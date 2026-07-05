variable "aws_region" {
  description = "AWS region to deploy into."
  type        = string
  default     = "us-east-1"
}

variable "name_prefix" {
  description = "Prefix for resource names and tags."
  type        = string
  default     = "rustc-perf"
}

variable "instance_type" {
  description = "EC2 instance type for the site host."
  type        = string
  default     = "t3.large"
}

variable "root_volume_size_gb" {
  description = "Root gp3 volume size in GiB. Holds the OS, Docker data, and the SQLite database."
  type        = number
  default     = 100
}

variable "site_container_image" {
  description = "Digest-pinned image for the rustc-perf container, e.g. <registry>/rustc-perf@sha256:<digest>. Leave empty to create just the ECR repository first (terraform apply -target=aws_ecr_repository.site), then set it and re-apply."
  type        = string
  default     = ""
}

variable "site_hostname" {
  description = "Optional public hostname served over HTTPS by Caddy. Point it at the Elastic IP in external DNS. Leave null to serve plain HTTP on the Elastic IP."
  type        = string
  default     = null
}
