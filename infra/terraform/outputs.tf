output "site_url" {
  description = "Primary URL for the site."
  value       = local.site_hostname == "" ? "http://${aws_eip.site.public_ip}" : "https://${local.site_hostname}"
}

# deploy.sh reads this back so its AWS CLI calls target the stack's region.
output "aws_region" {
  description = "AWS region the stack is deployed in."
  value       = var.aws_region
}

output "site_public_ip" {
  description = "Elastic IP attached to the site instance. Point external DNS here."
  value       = aws_eip.site.public_ip
}

output "instance_id" {
  description = "EC2 instance ID for the site host."
  value       = aws_instance.site.id
}

output "ecr_repository_url" {
  description = "ECR repository URL for the site image."
  value       = aws_ecr_repository.site.repository_url
}

output "backup_bucket_name" {
  description = "S3 bucket holding julia.db/.state backup archives."
  value       = aws_s3_bucket.backups.bucket
}

# deploy.sh passes these to `docker build` so the container user matches the
# UID/GID that owns the host data directory.
output "container_runtime_uid" {
  description = "UID the container runtime user is created with."
  value       = local.container_runtime_uid
}

output "container_runtime_gid" {
  description = "GID the container runtime user is created with."
  value       = local.container_runtime_gid
}
