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
runtime_uid="$(terraform output -raw container_runtime_uid)"
runtime_gid="$(terraform output -raw container_runtime_gid)"
registry="${ecr_url%%/*}"
repo="${ecr_url##*/}"
region="$(printf '%s' "$registry" | sed -E 's/.*\.ecr\.([^.]+)\.amazonaws\.com$/\1/')"
tag="deploy-$(date -u +%Y%m%dT%H%M%SZ)-$(git -C "$repo_root" rev-parse --short HEAD 2>/dev/null || echo nogit)"

# Kick the backup off first: send-command returns immediately, so the backup
# (snapshot + gzip + upload, minutes on a large database) runs on the instance
# while the image builds locally. It is polled to completion below, before
# anything is applied.
backup_cmd_id=""
if [ "${SKIP_BACKUP:-0}" != "1" ]; then
  instance_id="$(terraform output -raw instance_id 2>/dev/null || true)"
  if [ -n "$instance_id" ] && [ "$instance_id" != "null" ]; then
    echo "==> Starting pre-deploy backup on $instance_id (runs during the build)"
    backup_cmd_id="$(aws ssm send-command --region "$region" --instance-ids "$instance_id" \
      --document-name AWS-RunShellScript \
      --parameters 'commands=["sudo /usr/local/bin/rustc-perf-backup"]' \
      --query 'Command.CommandId' --output text)"
  fi
fi

echo "==> Logging in to ECR ($registry)"
aws ecr get-login-password --region "$region" | docker login --username AWS --password-stdin "$registry"

echo "==> Building image for linux/amd64: $ecr_url:$tag"
docker build --platform linux/amd64 \
  --build-arg RUSTC_PERF_RUNTIME_UID="$runtime_uid" \
  --build-arg RUSTC_PERF_RUNTIME_GID="$runtime_gid" \
  -t "$ecr_url:$tag" "$repo_root"

echo "==> Pushing image"
docker push "$ecr_url:$tag"

echo "==> Resolving image digest"
digest="$(aws ecr describe-images --region "$region" --repository-name "$repo" \
  --image-ids imageTag="$tag" --query 'imageDetails[0].imageDigest' --output text)"
image_ref="${ecr_url}@${digest}"
echo "    $image_ref"

if [ -n "$backup_cmd_id" ]; then
  echo "==> Waiting for the pre-deploy backup"
  # Poll until the command reaches a terminal state. `aws ssm wait` is not
  # usable here: its polling budget is fixed at 100 seconds, and the backup
  # takes minutes and grows with the database.
  deadline=$((SECONDS + 900))
  status="Pending"
  while [ "$SECONDS" -lt "$deadline" ]; do
    # The invocation may not exist for a moment right after send-command.
    status="$(aws ssm get-command-invocation --region "$region" --command-id "$backup_cmd_id" \
      --instance-id "$instance_id" --query 'Status' --output text 2>/dev/null || echo Pending)"
    case "$status" in
      Success|Failed|Cancelled|TimedOut|Undeliverable|Terminated) break ;;
    esac
    sleep 10
  done
  if [ "$status" != "Success" ]; then
    echo "Pre-deploy backup failed (status '$status'). Aborting: replacing the instance now would roll the database back to the last daily backup." >&2
    echo "Fix the backup (or run SKIP_BACKUP=1 ./deploy.sh to accept that data loss) and retry." >&2
    exit 1
  fi
fi

echo "==> Deploying (terraform apply)"
terraform apply -var "site_container_image=${image_ref}" "$@"
