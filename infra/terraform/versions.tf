terraform {
  required_version = ">= 1.6.0"

  # State is local. If more than one person ever applies, add a remote S3 backend
  # with locking here and run `terraform init -migrate-state`.

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.50"
    }
  }
}

provider "aws" {
  region = var.aws_region
}