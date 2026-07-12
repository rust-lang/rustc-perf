# Values live in the committed terraform.tfvars; fixed settings that are not
# worth varying are locals in main.tf.

variable "aws_region" {
  description = "AWS region to deploy into."
  type        = string
}

variable "name_prefix" {
  description = "Prefix for resource names and tags."
  type        = string
}

variable "instance_type" {
  description = "EC2 instance type for the site host."
  type        = string
}

variable "root_volume_size_gb" {
  description = "Root gp3 volume size in GiB. Holds the OS, Docker data, and the SQLite database."
  type        = number
}

variable "site_container_image" {
  description = "Digest-pinned image for the rustc-perf container, e.g. <registry>/rustc-perf@sha256:<digest>. Deliberately not set in terraform.tfvars: deploy.sh passes the freshly pushed digest as -var, and the instance precondition rejects anything else."
  type        = string
  default     = ""
}

variable "site_hostname" {
  description = "Optional public hostname served over HTTPS by Caddy. Point it at the Elastic IP in external DNS. Leave null to serve plain HTTP on the Elastic IP."
  type        = string
  default     = null
}
