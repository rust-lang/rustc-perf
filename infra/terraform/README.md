# AWS Terraform Deployment

This directory provisions a deliberately small rustc-perf deployment on AWS:

- One EC2 instance running the production rustc-perf container in Docker, in a minimal dedicated VPC (one public subnet).
- One Elastic IP attached directly to the site instance.
- One Caddy reverse proxy on the site instance for HTTP and automatic Let's Encrypt HTTPS.
- One encrypted gp3 root volume that stores the OS, Docker data, the SQLite file, and the orchestrator checkpoint state.
- One ECR repository for the site image.
- One private, encrypted S3 bucket for runtime backups of `julia.db` and `.state`.
- Systems Manager Session Manager access as the default debugging and operational shell path.

The container entrypoint is [run.jl](../../run.jl) invoked with `--install`. It starts the Julia orchestrator, launches the embedded `prod_site` Rust binary, and keeps ingesting fresh data over time.

For the end-to-end operational checklist, including first bring-up, backups,
restore, and the deploy flow, see
[docs/production-deployment.md](../../docs/production-deployment.md).

## Why this shape

- SQLite wants a normal local filesystem. EFS is the wrong default.
- One EC2 host is easy to understand, inspect, and roll back.
- An Elastic IP plus Caddy keeps the public endpoint stable without depending on Route53, ACM, CloudFront, or an ALB.
- The network is a minimal dedicated VPC: one public subnet and an internet gateway, nothing else. Internet exposure is controlled entirely by the security group (only 80/443).
- Deploys are immutable: the image digest and every host setting live in `user_data`, so a new version is shipped by running `terraform apply`, which replaces the instance. There is nothing to drift, because the running machine is always exactly what the code describes.
- Instance replacement is non-destructive: on boot a fresh host restores `julia.db`, `.state`, and Caddy's TLS certificates from the latest S3 backup, so it keeps the dataset and does not re-request certificates.
- Runtime backups go to a private S3 bucket on a daily timer. They use SQLite's online `.backup`, so they do not stop the service.
- The host boot path is hardened enough for a public test deployment: IMDSv2 is required, Caddy terminates TLS on the host, and the container runs as a fixed non-root UID/GID with `no-new-privileges` and no Linux capabilities.

## Architecture

- The EC2 instance lives in the stack's public subnet with an Elastic IP so it can install packages, pull container images, and serve the public site without adding NAT.
- The instance security group exposes only ports 80 and 443 publicly. Operator access is through Session Manager; this stack does not provision SSH ingress.
- The SQLite file lives at `/var/lib/rustc-perf/julia.db` on the root gp3 volume.
- The Julia orchestrator checkpoint files live at `/var/lib/rustc-perf/.state` and are persisted alongside the database.
- The active image digest is baked into `user_data` by Terraform and recorded in the host env file the container runner reads.
- The container's runtime environment is fixed and non-secret, written once by cloud-init.
- The backup timer uploads tarballs containing `julia.db`, `.state/`, and a small metadata file to S3.
- Browser traffic reaches Caddy on the instance over HTTP and HTTPS. When `site_hostname` is set and points at the Elastic IP, Caddy provisions Let's Encrypt certificates automatically.
- The deployment expects the mounted data directory to stay private to the service: Terraform and cloud-init set the directory to `0750` and the SQLite file to `0640` for the container runtime user.

## State And Instance Lifecycle

State lives in a local `terraform.tfstate`, which is fine for a single operator running applies from one machine. Losing it is recoverable (re-import the ~15 resources; no data is at risk, since the database and certificates live in the S3 backup bucket). If more than one person will ever apply, add a remote S3 backend with locking in [versions.tf](versions.tf) and run `terraform init -migrate-state`.

`user_data_replace_on_change = true`, so any change to the image or host configuration replaces the instance on the next `terraform apply`. That is the intended deploy mechanism, not an accident: the new host restores `julia.db`, `.state`, and Caddy's certificates from the latest S3 backup on boot. The daily backup means a replacement loses at most the last day of ingested data (which the orchestrator re-derives), and `deploy.sh` takes a fresh pre-deploy backup automatically — see [docs/production-deployment.md](../../docs/production-deployment.md).

## Prerequisites

1. Terraform 1.6 or newer.
2. AWS credentials with permission to create EC2, VPC, Elastic IP, ECR, S3, and IAM (create-role) resources. The instance profile is what gives the host Session Manager access and its S3 backup permissions. Note that the PowerUserAccess permission set cannot manage IAM.
3. A container image built from the repository root [Dockerfile](../../Dockerfile).

The stack creates its own small VPC (one public subnet); no default VPC is required.

### Credentials note for `aws login`

The Terraform AWS provider cannot read credentials from the AWS CLI's browser-based `aws login` flow ([terraform-provider-aws#45316](https://github.com/hashicorp/terraform-provider-aws/issues/45316)). If that is how you authenticate, add a bridge profile to `~/.aws/config` that resolves credentials through the CLI, and run Terraform (and `deploy.sh`) with it:

```ini
[profile terraform]
credential_process = aws configure export-credentials --profile default --format process
region = us-east-1
```

```bash
AWS_PROFILE=terraform ./deploy.sh
```

## Minimal first deploy

This stack exposes the site through the instance Elastic IP and, when `site_hostname` is set, through a stable HTTPS hostname served by Caddy.

1. Copy [terraform.tfvars.example](terraform.tfvars.example) to `terraform.tfvars`.
2. Set `site_container_image` to the first digest-pinned image reference. It is baked into the instance's `user_data`.
3. Set `site_hostname` to the public hostname you will point at the instance Elastic IP.
4. Use the generated `site_public_ip` output to create the external DNS `A` record.
5. Run:

```bash
terraform init
terraform plan
terraform apply
```

6. Point your DNS record at `site_public_ip`, wait for propagation, then open the `site_url` output.

## Optional ECR bootstrap

If you want Terraform to create the ECR repository before the instance exists, run:

```bash
terraform apply -target=aws_ecr_repository.site
terraform output -raw ecr_repository_url
```

Then build and push the image, set `site_container_image`, and run the full apply.

Example build and push flow:

```bash
aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin 123456789012.dkr.ecr.us-east-1.amazonaws.com
docker build -t rustc-perf:latest ../..
docker tag rustc-perf:latest 123456789012.dkr.ecr.us-east-1.amazonaws.com/rustc-perf-site:latest
docker push 123456789012.dkr.ecr.us-east-1.amazonaws.com/rustc-perf-site:latest
```

The EC2 instance will log in to ECR automatically before pulling the image when `site_container_image` points at an ECR registry.

The ECR repository is created with immutable tags so a later push cannot silently replace an already referenced image tag.

After the first apply, ship new versions with `./deploy.sh`, which builds, pushes, and applies with the new digest (see "Deploying a new version" below).

## Stable Hostname And HTTPS

Set `site_hostname` to the stable public hostname you want to serve, then point that name at `site_public_ip` in your external DNS provider.

Caddy runs on the instance and obtains Let's Encrypt certificates automatically once the hostname resolves to the Elastic IP. The generated `site_url` output switches to `https://<site_hostname>` when `site_hostname` is configured.

## Existing Data

This stack no longer automates local SQLite uploads or provisions SSH for bootstrap.
If you need to move existing runtime data onto a fresh host, do the first apply,
then restore `julia.db` and `.state/` manually using the procedure in
[docs/production-deployment.md](../../docs/production-deployment.md).

## Session Manager Access

Use Session Manager as the normal operator access path. The Terraform stack
attaches an EC2 instance profile with `AmazonSSMManagedInstanceCore` directly to
the EC2 instance, so you do not need SSH for routine debugging.

For the step-by-step flow and a registration check, see
[docs/production-deployment.md](../../docs/production-deployment.md).

## Deploying a new version

Run the deploy script from this directory:

```bash
./deploy.sh
```

It builds the image (for `linux/amd64`), pushes it to ECR, takes a pre-deploy backup, and runs `terraform apply` with the new digest — which replaces the instance. The new host restores the database and TLS certificates from the latest backup on boot. Terraform will show the plan and prompt before it replaces anything; pass `-auto-approve` (or any other `terraform apply` flag) through: `./deploy.sh -auto-approve`. If the pre-deploy backup fails, the deploy aborts; set `SKIP_BACKUP=1` to skip the backup and accept rolling back to the last daily one.

Expect a couple of minutes of downtime while the new instance boots, restores, and Julia warms up.

Changing any host setting works the same way — edit the code and run `./deploy.sh`. The instance is rebuilt from `user_data`, so the running machine never drifts from what the code says. A bare `terraform apply` does not work: the image digest is only ever passed by `deploy.sh` as `-var site_container_image=...` (it is deliberately not persisted in `terraform.tfvars`), so an apply without it fails the digest precondition.

If you prefer to do it by hand, the script is just: `docker build --platform linux/amd64` → `docker push` → resolve the `@sha256` digest → `terraform apply -var "site_container_image=<ecr-url>@<digest>"`. Run `sudo /usr/local/bin/rustc-perf-backup` on the instance first — a manual apply that replaces the instance restores whatever the latest backup is.

## Backups And Restore

Terraform creates a private, encrypted S3 bucket and installs a `rustc-perf-backup.timer` systemd timer on the instance.

Backups use SQLite's online `.backup` API and do not stop the service. Each run uploads a timestamped archive under `<prefix>/archive/<hostname>/` and refreshes `<prefix>/latest.tar.gz`, an independent full copy of that archive (not a reference to it). Timestamped archives expire after 30 days; `latest.tar.gz` never expires, so a replacement host always has a restore source even if backups have been failing.

Each uploaded archive contains:

- `julia.db`
- `.state/`
- `metadata.txt` with a timestamp, hostname, and image reference

A fresh or replaced host restores itself automatically: on service start, `rustc-perf-restore-if-empty` seeds `julia.db` and `.state` when the database is missing, and Caddy's certificate store when it is missing, so existing data is never overwritten. To restore a specific older archive over the current data, follow the manual procedure in [docs/production-deployment.md](../../docs/production-deployment.md).

## How to test the deployment

1. Check the generated URL:

```bash
terraform output -raw site_url
```

2. If the page does not load, inspect the instance with Session Manager:

```bash
sudo systemctl status rustc-perf-site
sudo systemctl status rustc-perf-caddy
sudo journalctl -u rustc-perf-site -n 100 --no-pager
sudo journalctl -u rustc-perf-caddy -n 100 --no-pager
sudo docker ps
sudo docker logs rustc-perf-site --tail 100
sudo docker logs rustc-perf-caddy --tail 100
ls -lh /var/lib/rustc-perf
```

Useful things to confirm on-host:

- `/var/lib/rustc-perf/julia.db` exists and is non-empty.
- `/var/lib/rustc-perf/.state/` contains both checkpoint files.
- The container is listening on `127.0.0.1:2346` and Caddy is listening on ports 80 and 443.

3. Use Session Manager for operator access; no SSH ingress is provisioned by this stack.

## After the first successful deploy

Once the basic path is working, the next sensible upgrades are:

1. Keep the production verification URL pointed at the stable HTTPS hostname rather than the raw Elastic IP.
2. Exercise a full restore from the S3 backup bucket before relying on the stack for production recovery.

This stack is intentionally biased toward clarity over flexibility.