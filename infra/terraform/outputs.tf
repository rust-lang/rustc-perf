output "site_url" {
  description = "Primary URL for the site."
  value       = "${local.site_url_scheme}://${local.public_site_hostname}"
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
