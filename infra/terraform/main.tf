locals {
  common_tags = {
    Project     = "rustc-perf"
    ManagedBy   = "Terraform"
    Environment = "production"
  }

  # Fixed settings that are not worth exposing as variables. Change them here.
  data_mount_path  = "/var/lib/rustc-perf"
  db_filename      = "julia.db"
  container_port   = 2346
  datasette_port   = 8001
  datasette_image  = "datasetteproject/datasette:0.65.2"
  caddy_image      = "caddy:2.8.4-alpine"
  caddy_data_dir   = "/var/lib/caddy/data"
  caddy_config_dir = "/var/lib/caddy/config"
  # Owned here; deploy.sh passes these to `docker build` so the container user
  # is created with the same UID/GID that owns the host data directory. The
  # Dockerfile's ARG defaults only matter for images built outside deploy.sh.
  container_runtime_uid = 10001
  container_runtime_gid = 10001
  backup_prefix         = "runtime"
  # Timestamped archives live under this subdirectory of backup_prefix; the
  # lifecycle rule below and both backup scripts derive their layout from it.
  backup_archive_dir    = "archive"
  backup_latest_name    = "latest.tar.gz"
  backup_retention_days = 30
  # Every deploy.sh run pushes an image before its abort points (failed
  # pre-deploy backup, rejected plan), so keep enough headroom that a run of
  # aborted deploys cannot expire the digest the live instance still runs.
  ecr_max_images = 10

  ecr_repository_name = "${var.name_prefix}-site"
  backup_bucket_name  = lower("${var.name_prefix}-${data.aws_caller_identity.current.account_id}-${var.aws_region}-backups")

  site_image_ref = trimspace(var.site_container_image)
  # Empty when no hostname is configured; the site is then plain HTTP on the EIP.
  site_hostname = trimspace(coalesce(var.site_hostname, ""))
}

data "aws_caller_identity" "current" {}

# A minimal dedicated VPC (the account has no default VPC): one public subnet
# with a route to an internet gateway. What is reachable from the internet is
# governed by aws_security_group.instance below (only 80/443).
resource "aws_vpc" "this" {
  cidr_block           = "10.42.0.0/16"
  enable_dns_support   = true
  enable_dns_hostnames = true

  tags = { Name = "${var.name_prefix}-vpc" }
}

resource "aws_internet_gateway" "this" {
  vpc_id = aws_vpc.this.id
  tags   = { Name = "${var.name_prefix}-igw" }
}

resource "aws_subnet" "public" {
  vpc_id                  = aws_vpc.this.id
  cidr_block              = "10.42.1.0/24"
  map_public_ip_on_launch = true

  tags = { Name = "${var.name_prefix}-public" }
}

resource "aws_route_table" "public" {
  vpc_id = aws_vpc.this.id

  route {
    cidr_block = "0.0.0.0/0"
    gateway_id = aws_internet_gateway.this.id
  }

  tags = { Name = "${var.name_prefix}-public" }
}

resource "aws_route_table_association" "public" {
  subnet_id      = aws_subnet.public.id
  route_table_id = aws_route_table.public.id
}

data "aws_ssm_parameter" "al2023_ami" {
  name = "/aws/service/ami-amazon-linux-latest/al2023-ami-kernel-default-x86_64"
}

resource "aws_security_group" "instance" {
  name = "${var.name_prefix}-instance"
  # Immutable in EC2: changing this forces SG (and instance) replacement.
  description = "Ingress for the rustc-perf site instance"
  vpc_id      = aws_vpc.this.id

  ingress {
    description = "HTTP"
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    description = "HTTPS"
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = { Name = "${var.name_prefix}-instance" }
}

resource "aws_ecr_repository" "site" {
  name                 = local.ecr_repository_name
  image_tag_mutability = "IMMUTABLE"

  image_scanning_configuration {
    scan_on_push = true
  }

  encryption_configuration {
    encryption_type = "AES256"
  }

  tags = { Name = local.ecr_repository_name }
}

resource "aws_ecr_lifecycle_policy" "site" {
  repository = aws_ecr_repository.site.name
  policy = jsonencode({
    rules = [{
      rulePriority = 1
      description  = "Expire all but the newest ${local.ecr_max_images} images"
      selection = {
        tagStatus   = "any"
        countType   = "imageCountMoreThan"
        countNumber = local.ecr_max_images
      }
      action = { type = "expire" }
    }]
  })
}

# Private, encrypted, auto-expiring bucket for julia.db/.state backups.
resource "aws_s3_bucket" "backups" {
  bucket = local.backup_bucket_name
  tags   = { Name = "${var.name_prefix}-backups" }
}

resource "aws_s3_bucket_public_access_block" "backups" {
  bucket                  = aws_s3_bucket.backups.id
  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}

# Reject any non-TLS access to the backups.
resource "aws_s3_bucket_policy" "backups" {
  bucket = aws_s3_bucket.backups.id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Sid       = "DenyInsecureTransport"
      Effect    = "Deny"
      Principal = "*"
      Action    = "s3:*"
      Resource = [
        aws_s3_bucket.backups.arn,
        "${aws_s3_bucket.backups.arn}/*",
      ]
      Condition = {
        Bool = { "aws:SecureTransport" = "false" }
      }
    }]
  })
}

resource "aws_s3_bucket_server_side_encryption_configuration" "backups" {
  bucket = aws_s3_bucket.backups.id

  rule {
    apply_server_side_encryption_by_default {
      sse_algorithm = "AES256"
    }
  }
}

resource "aws_s3_bucket_lifecycle_configuration" "backups" {
  bucket = aws_s3_bucket.backups.id

  # Only the timestamped archives expire. latest.tar.gz — an independent full
  # copy of the newest archive, not a reference to it — lives outside archive/
  # and must never expire: it is the sole data source for a replaced instance,
  # and it has to survive a stretch of failed or missed backups longer than
  # the retention window.
  rule {
    id     = "expire-old-runtime-backups"
    status = "Enabled"

    filter {
      prefix = "${local.backup_prefix}/${local.backup_archive_dir}/"
    }

    expiration {
      days = local.backup_retention_days
    }
  }

  rule {
    id     = "abort-incomplete-uploads"
    status = "Enabled"
    filter {}

    abort_incomplete_multipart_upload {
      days_after_initiation = 7
    }
  }
}

resource "aws_iam_role" "instance" {
  name = "${var.name_prefix}-instance-role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Effect    = "Allow"
      Action    = "sts:AssumeRole"
      Principal = { Service = "ec2.amazonaws.com" }
    }]
  })
}

# Session Manager access (operator shell, and the SSM Run Command deploy path).
resource "aws_iam_role_policy_attachment" "ssm_core" {
  role       = aws_iam_role.instance.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonSSMManagedInstanceCore"
}

# App permissions: pull from ECR and read/write the S3 backup archives.
resource "aws_iam_role_policy" "app" {
  name = "${var.name_prefix}-instance-app"
  role = aws_iam_role.instance.id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect   = "Allow"
        Action   = ["ecr:GetAuthorizationToken"]
        Resource = "*"
      },
      {
        Effect = "Allow"
        Action = [
          "ecr:BatchCheckLayerAvailability",
          "ecr:BatchGetImage",
          "ecr:GetDownloadUrlForLayer",
        ]
        Resource = aws_ecr_repository.site.arn
      },
      {
        Effect   = "Allow"
        Action   = ["s3:GetBucketLocation", "s3:ListBucket"]
        Resource = aws_s3_bucket.backups.arn
      },
      {
        Effect   = "Allow"
        Action   = ["s3:AbortMultipartUpload", "s3:GetObject", "s3:PutObject", "s3:GetObjectTagging"]
        Resource = "${aws_s3_bucket.backups.arn}/${local.backup_prefix}/*"
      },
    ]
  })
}

resource "aws_iam_instance_profile" "instance" {
  name = "${var.name_prefix}-instance-profile"
  role = aws_iam_role.instance.name
}

# The files cloud-init places on the host. The scripts and units are plain files
# under files/; the three that need Terraform values are rendered from templates.
locals {
  user_data_files = {
    "/usr/local/bin/rustc-perf-run-caddy"        = { mode = "0755", content = file("${path.module}/files/rustc-perf-run-caddy") }
    "/usr/local/bin/rustc-perf-run-container"    = { mode = "0755", content = file("${path.module}/files/rustc-perf-run-container") }
    "/usr/local/bin/rustc-perf-run-datasette"    = { mode = "0755", content = file("${path.module}/files/rustc-perf-run-datasette") }
    "/usr/local/bin/rustc-perf-backup"           = { mode = "0755", content = file("${path.module}/files/rustc-perf-backup") }
    "/usr/local/bin/rustc-perf-restore-if-empty" = { mode = "0755", content = file("${path.module}/files/rustc-perf-restore-if-empty") }

    "/etc/systemd/system/rustc-perf-site.service"      = { mode = "0644", content = file("${path.module}/files/rustc-perf-site.service") }
    "/etc/systemd/system/rustc-perf-datasette.service" = { mode = "0644", content = file("${path.module}/files/rustc-perf-datasette.service") }
    "/etc/systemd/system/rustc-perf-caddy.service"     = { mode = "0644", content = file("${path.module}/files/rustc-perf-caddy.service") }
    "/etc/systemd/system/rustc-perf-backup.service"    = { mode = "0644", content = file("${path.module}/files/rustc-perf-backup.service") }
    "/etc/systemd/system/rustc-perf-backup.timer"      = { mode = "0644", content = file("${path.module}/files/rustc-perf-backup.timer") }

    "/etc/rustc-perf-site.host.env" = { mode = "0600", content = templatefile("${path.module}/files/host.env.tftpl", {
      aws_region            = var.aws_region
      container_port        = local.container_port
      datasette_port        = local.datasette_port
      data_mount_path       = local.data_mount_path
      db_filename           = local.db_filename
      site_image_ref        = local.site_image_ref
      container_runtime_uid = local.container_runtime_uid
      container_runtime_gid = local.container_runtime_gid
      backup_bucket_name    = aws_s3_bucket.backups.bucket
      backup_prefix         = local.backup_prefix
      backup_archive_dir    = local.backup_archive_dir
      backup_latest_name    = local.backup_latest_name
      caddy_image           = local.caddy_image
      caddy_data_dir        = local.caddy_data_dir
      caddy_config_dir      = local.caddy_config_dir
      datasette_image       = local.datasette_image
    }) }

    "/etc/caddy/Caddyfile" = { mode = "0644", content = templatefile("${path.module}/files/Caddyfile.tftpl", {
      public_ip       = aws_eip.site.public_ip
      public_hostname = local.site_hostname
      container_port  = local.container_port
      datasette_port  = local.datasette_port
    }) }
  }
}

resource "aws_instance" "site" {
  ami                         = data.aws_ssm_parameter.al2023_ami.value
  instance_type               = var.instance_type
  subnet_id                   = aws_subnet.public.id
  vpc_security_group_ids      = [aws_security_group.instance.id]
  associate_public_ip_address = true
  iam_instance_profile        = aws_iam_instance_profile.instance.name

  # Deploys are immutable: the image digest and every host setting live in this
  # user_data, so `terraform apply` with a change replaces the instance. On boot
  # the new host restores julia.db/.state (and Caddy's TLS certs) from the latest
  # S3 backup, so a replacement does not lose data or re-issue certificates.
  user_data_replace_on_change = true

  # Gzipped because the rendered cloud-init (with every script inlined) sits
  # close to EC2's 16 KB user_data limit; cloud-init unpacks it transparently.
  user_data_base64 = base64gzip(templatefile("${path.module}/cloud-init.yaml.tftpl", {
    files                 = local.user_data_files
    container_runtime_uid = local.container_runtime_uid
    container_runtime_gid = local.container_runtime_gid
    data_mount_path       = local.data_mount_path
    caddy_data_dir        = local.caddy_data_dir
    caddy_config_dir      = local.caddy_config_dir
  }))

  depends_on = [
    aws_iam_role_policy_attachment.ssm_core,
    aws_iam_role_policy.app,
    aws_s3_bucket_public_access_block.backups,
  ]

  lifecycle {
    # The SSM parameter tracks the newest AL2023 AMI, which changes every few
    # weeks. Without this, a routine `terraform apply` would replace the
    # instance (and roll the database back to the last backup) just because a
    # new AMI was published. New AMIs are still picked up whenever the
    # instance is replaced for a real reason (any deploy).
    ignore_changes = [ami]

    precondition {
      condition     = length(regexall("@sha256:[0-9a-fA-F]{64}$", local.site_image_ref)) > 0
      error_message = "site_container_image must be set and pinned to an immutable @sha256 digest, e.g. <registry>/rustc-perf@sha256:<digest>. Create the ECR repository first with -target=aws_ecr_repository.site if you need to build the image before the full apply."
    }
  }

  root_block_device {
    volume_size = var.root_volume_size_gb
    volume_type = "gp3"
    encrypted   = true
  }

  metadata_options {
    http_endpoint = "enabled"
    http_tokens   = "required" # require IMDSv2
  }

  tags = { Name = "${var.name_prefix}-site" }
}

resource "aws_eip" "site" {
  domain = "vpc"
  tags   = { Name = "${var.name_prefix}-site-eip" }
}

resource "aws_eip_association" "site" {
  allocation_id = aws_eip.site.id
  instance_id   = aws_instance.site.id
}
