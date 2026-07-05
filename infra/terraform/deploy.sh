#!/usr/bin/env bash
#
# Build the site image, push it to ECR, and deploy it.
#
# Deploys are immutable: this pushes a fresh image and runs `terraform apply`
# with its digest, which replaces the instance. The new host restores the
# database and TLS certificates from the latest S3 backup on boot.
#
# Usage:
#   ./deploy.sh                 # build, push, back up, apply (prompts to confirm)
#   ./deploy.sh -auto-approve   # extra args are passed through to `terraform apply`
#   SKIP_BACKUP=1 ./deploy.sh   # skip the pre-deploy backup
#
# Requires docker, terraform, and the AWS CLI with credentials for this account.
# The stack must already be applied once (so the ECR repo exists); see README.md
# for first-time bootstrap.

set -euo pipefail

script_dir="$(cd "$(dirname "$0")" && pwd)"
repo_root="$(cd "$script_dir/../.." && pwd)"
cd "$script_dir"

echo "==> Reading Terraform outputs"
ecr_url="$(terraform output -raw ecr_repository_url 2>/dev/null || true)"
if [ -z "$ecr_url" ] || [ "$ecr_url" = "null" ]; then
  echo "Could not read ecr_repository_url. Apply the stack once first (see README.md)." >&2
  exit 1
fi
registry="${ecr_url%%/*}"
repo="${ecr_url##*/}"
region="$(printf '%s' "$registry" | sed -E 's/.*\.ecr\.([^.]+)\.amazonaws\.com$/\1/')"
tag="deploy-$(date -u +%Y%m%dT%H%M%SZ)-$(git -C "$repo_root" rev-parse --short HEAD 2>/dev/null || echo nogit)"

echo "==> Logging in to ECR ($registry)"
aws ecr get-login-password --region "$region" | docker login --username AWS --password-stdin "$registry"

echo "==> Building image for linux/amd64: $ecr_url:$tag"
docker build --platform linux/amd64 -t "$ecr_url:$tag" "$repo_root"

echo "==> Pushing image"
docker push "$ecr_url:$tag"

echo "==> Resolving image digest"
digest="$(aws ecr describe-images --region "$region" --repository-name "$repo" \
  --image-ids imageTag="$tag" --query 'imageDetails[0].imageDigest' --output text)"
image_ref="${ecr_url}@${digest}"
echo "    $image_ref"

if [ "${SKIP_BACKUP:-0}" != "1" ]; then
  instance_id="$(terraform output -raw instance_id 2>/dev/null || true)"
  if [ -n "$instance_id" ] && [ "$instance_id" != "null" ]; then
    echo "==> Taking a pre-deploy backup on $instance_id"
    cmd_id="$(aws ssm send-command --region "$region" --instance-ids "$instance_id" \
      --document-name AWS-RunShellScript \
      --parameters 'commands=["sudo /usr/local/bin/rustc-perf-backup"]' \
      --query 'Command.CommandId' --output text)"
    aws ssm wait command-executed --region "$region" --command-id "$cmd_id" --instance-id "$instance_id" || true
    status="$(aws ssm get-command-invocation --region "$region" --command-id "$cmd_id" \
      --instance-id "$instance_id" --query 'Status' --output text 2>/dev/null || echo Unknown)"
    if [ "$status" != "Success" ]; then
      echo "    Warning: backup status was '$status'; continuing. The new host will fall back to the latest daily backup." >&2
    fi
  fi
fi

echo "==> Deploying (terraform apply)"
terraform apply -var "site_container_image=${image_ref}" "$@"
