# Production Deployment Runbook

This document is the source of truth for bootstrapping the stack, operating
backups, deploying new versions, and restoring the service.

## Current Model

The production setup is intentionally simple:

- One EC2 instance runs the production container.
- One Elastic IP attaches directly to the instance.
- One optional external DNS `A` record points at that Elastic IP.
- One Caddy reverse proxy runs on the instance and terminates HTTP or HTTPS.
- The image digest and all host configuration live in `user_data`.
- The container's runtime environment is fixed and non-secret, written once on the host.
- One encrypted S3 bucket stores `julia.db`, `.state`, and Caddy's TLS certificates.

Deploys are immutable and manual. You ship a new version by running
`terraform apply` with a new image digest, which replaces the instance. On
boot the new host restores `julia.db`, `.state`, and the certificate store from
the latest S3 backup, so nothing is lost and no certificate is re-requested.
Because the whole machine is rebuilt from `user_data` on every apply, the
running host never drifts from what the code describes.

A replacement restores from the most recent backup, so the daily timer bounds
the loss to about a day of ingested data (which the orchestrator re-derives).
To lose nothing, take a fresh backup right before applying:

```bash
INSTANCE_ID="$(cd infra/terraform && terraform output -raw instance_id)"
aws ssm send-command \
  --instance-ids "$INSTANCE_ID" \
  --document-name AWS-RunShellScript \
  --parameters 'commands=["sudo /usr/local/bin/rustc-perf-backup"]'
```

## What You Need To Do

The shortest bring-up checklist, in order:

1. Decide the production hostname and make sure you can create an external DNS `A` record for it.
2. Build and push the first production image and record its digest.
3. Fill in `infra/terraform/terraform.tfvars` with the image digest and hostname.
4. Run the full Terraform apply.
5. Record the Terraform outputs you will need later.
6. Create the DNS `A` record pointing at `site_public_ip`.
7. Verify the public URL.

The remaining sections expand each of those steps.

## Before You Start

Collect or choose these values before you begin:

- AWS account ID
- AWS region
- Terraform `name_prefix`
- Production hostname, for example `perf.example.com`

You also need a default VPC in the region and AWS credentials that can create
the Terraform resources used by the stack.

## Bootstrap Once

This is the exact sequence for the first real production bring-up.

### 1. Create Or Reuse The Image Repository

If you are using the Terraform-managed ECR repository, create it first:

```bash
cd infra/terraform
terraform init
terraform apply -target=aws_ecr_repository.site
terraform output -raw ecr_repository_url
```

If you are using an existing registry, skip this step and use that repository
instead.

### 2. Build And Push The First Production Image

Build and push the first production image before the full Terraform apply.

If you created the ECR repository through Terraform, a practical bootstrap flow
is:

```bash
ECR_URL="$(cd infra/terraform && terraform output -raw ecr_repository_url)"
docker build -t rustc-perf:bootstrap .
docker tag rustc-perf:bootstrap "$ECR_URL:bootstrap"
docker push "$ECR_URL:bootstrap"
```

Then set `site_container_image` to the digest-pinned form of that image, not
the mutable `:bootstrap` tag.

### 3. Fill In terraform.tfvars For Production

At minimum, set these values:

- `site_container_image`
- `site_hostname`

If you are moving forward an existing dataset, plan to restore `julia.db` and
`.state/` manually after the first host comes up.

### 4. Run The First Full Apply

```bash
cd infra/terraform
terraform init
terraform plan
terraform apply
```

### 5. Record The Important Outputs

Useful outputs after the apply:

- `site_url`
- `site_public_ip` (create the DNS `A` record against this; useful before DNS propagates)
- `ecr_repository_url`
- `backup_bucket_name`

## Stable Hostname And HTTPS

Terraform exposes the site directly from the instance.

- `http://<site_public_ip>` works for direct testing.
- `https://<site_hostname>` works after you create the DNS `A` record and Caddy obtains a Let's Encrypt certificate.
- If `site_hostname` is not set, the stack stays in plain HTTP mode on the Elastic IP.

## Deploying A New Version

Deploys are manual and immutable. The `deploy.sh` script does the whole flow —
build the image, push it to ECR, take a pre-deploy backup, and `terraform apply`
with the new digest, which replaces the instance. The new host restores its data
and certificates from the latest backup on boot.

```bash
cd infra/terraform
./deploy.sh
```

Terraform prompts before replacing the instance. Pass `terraform apply` flags
through (`./deploy.sh -auto-approve`), or set `SKIP_BACKUP=1` to skip the
pre-deploy backup. Under the hood the script runs:

```bash
docker build --platform linux/amd64 -t "$ECR_URL:$TAG" ../..
docker push "$ECR_URL:$TAG"
DIGEST="$(aws ecr describe-images --repository-name <repo> --image-ids imageTag=$TAG \
  --query 'imageDetails[0].imageDigest' --output text)"
terraform apply -var "site_container_image=$ECR_URL@$DIGEST"
```

Expect a couple of minutes of downtime while the new instance boots, restores,
and Julia warms up. Any change to host configuration deploys the same way —
edit the code and run `./deploy.sh` (or `terraform apply`).

## Session Manager Access

Use Systems Manager Session Manager as the default operator access path for
this stack. You should not need to open port `22` for normal debugging or
day-2 operations.

This Terraform stack attaches an EC2 instance profile with
`AmazonSSMManagedInstanceCore` directly to the site instance. The AWS console
may still mention Default Host Management Configuration, but this deployment
does not depend on DHMC. The expected path is a per-instance IAM role plus the
SSM agent on the host.

Find the running instance and open a shell:

```bash
INSTANCE_ID="$(aws ec2 describe-instances \
  --filters \
    'Name=tag:Name,Values=rustc-perf-site' \
    'Name=instance-state-name,Values=running' \
  --query 'Reservations[0].Instances[0].InstanceId' \
  --output text)"

aws ssm start-session --target "$INSTANCE_ID"
```

If the instance does not appear in Session Manager yet:

1. Run `terraform plan` and `terraform apply` so the instance has the expected IAM instance profile.
2. Wait a minute or two for the instance to register after the profile is attached.
3. Confirm registration with `aws ssm describe-instance-information`.

Example registration check:

```bash
aws ssm describe-instance-information \
  --filters "Key=InstanceIds,Values=$INSTANCE_ID" \
  --query 'InstanceInformationList[0].PingStatus' \
  --output text
```

This stack does not provision SSH or Terraform-driven local SQLite uploads.
Use Session Manager plus the documented restore procedure if you need to bring
forward existing runtime data.

## Readiness And Warm-Up

The first time a fresh host starts `rustc-perf-site`, Julia package precompilation can take a couple of minutes. During that warm-up window:

- `rustc-perf-site` may show as active in systemd even though the HTTP service is not ready yet.
- `rustc-perf-caddy` may return `502 Bad Gateway` because the backend on `127.0.0.1:2346` is not answering normally yet.
- `docker logs rustc-perf-site` will show a long stream of `Precompiling packages...` output.

Treat the service as ready only when the backend is returning a normal HTTP response, not merely when the systemd unit is running.

Practical readiness checks on the instance:

```bash
sudo systemctl status rustc-perf-site
sudo docker logs rustc-perf-site --tail 100
curl -I http://127.0.0.1:2346/
curl -I http://127.0.0.1/
sudo ss -ltnp | grep -E ':(80|443|2346)\b'
```

What to look for:

- `curl -I http://127.0.0.1:2346/` returns `200 OK` or another normal HTTP response instead of `Empty reply from server` or connection resets.
- `curl -I http://127.0.0.1/` returns a normal response through Caddy instead of `502 Bad Gateway`.
- `docker logs rustc-perf-site` has moved past the long precompile output and stopped printing fresh package compilation lines.

## Runtime Environment

The container's runtime environment is fixed and non-secret. Cloud-init writes
`/etc/rustc-perf-site.container.env` once with the port, data directory, and
database filename. There are no secrets to resolve at start time. If the app
later needs a secret (for example a GitHub API token to avoid rate limits),
add it as a single SSM SecureString and wire it into the env file and the
instance IAM policy.

## Backup Model

- Terraform creates a private, encrypted S3 bucket.
- The instance gets `rustc-perf-backup.service` and a daily `rustc-perf-backup.timer`.
- You can also run the backup by hand before a deploy (see "Deploying A New Version").

Each backup archive contains:

- `julia.db`
- `.state/`
- `caddy-data/` — Caddy's TLS certificate store, so a replaced host reuses the existing certificate
- `metadata.txt` with a timestamp, hostname, and image reference

The backup script takes a consistent online snapshot with SQLite's `.backup`
API while the service keeps running, so backups no longer cause downtime. In
WAL mode `.backup` yields a single, already checkpointed database file, so the
`-wal`/`-shm` sidecars are not archived. The snapshot is staged on the instance
root volume rather than in `/tmp`, so large backups do not depend on a small
tmpfs. Each run uploads a timestamped archive under
`<prefix>/archive/<hostname>/` and then refreshes `<prefix>/latest.tar.gz` so
a fresh host can restore deterministically on boot. `latest.tar.gz` is an
independent full copy of the newest archive (an S3 server-side copy), not a
reference to it, so expiring the timestamped archives never invalidates it.
Only the timestamped archives are subject to the 30-day S3 lifecycle
expiration; `latest.tar.gz` never expires, so the restore source survives even
if backups stop running for longer than the retention window.

## Manual Backup And Inspection

On the instance:

```bash
sudo /usr/local/bin/rustc-perf-backup
systemctl status rustc-perf-backup.timer
sudo systemctl status rustc-perf-backup.service
sudo journalctl -u rustc-perf-backup.service -n 100 --no-pager
systemctl list-timers rustc-perf-backup.timer
```

From AWS:

```bash
aws s3 ls s3://<backup-bucket>/<backup-prefix>/ --recursive
```

If a backup fails, check both the backup service logs and root-volume free
space. The snapshot is staged under `/var/lib/rustc-perf-backup-staging`
before it is streamed to S3.

## Restore Procedure

A fresh host restores automatically. On service start, `rustc-perf-site` runs
`rustc-perf-restore-if-empty`, which seeds `julia.db` and `.state` from
`latest.tar.gz` (falling back to the newest timestamped archive) **only when no
local database is present**. An existing database is never overwritten, so this
is safe to run on every start. If the database is missing and no backup can be
restored (first bring-up with an empty bucket, or a download failure), the unit
fails and keeps retrying instead of starting the container without data — seed
`julia.db` (or fix the backups) and restart `rustc-perf-site`. The manual
procedure below is for restoring a specific, older archive over the current
data.

1. Identify the backup object you want to restore.
2. Stop the service:

```bash
sudo systemctl stop rustc-perf-site
```

3. Download and unpack the archive on the host.
4. Replace the live runtime data under `data_mount_path`.
5. Restore the expected ownership and permissions:

```bash
sudo chown -R 10001:10001 /var/lib/rustc-perf
sudo chmod 750 /var/lib/rustc-perf /var/lib/rustc-perf/.state
sudo chmod 640 /var/lib/rustc-perf/julia.db
```

6. Start the service again:

```bash
sudo systemctl start rustc-perf-site
sudo systemctl restart rustc-perf-caddy
sudo systemctl is-active rustc-perf-site
```

7. Verify the backend and public URL both return normal HTTP responses.

If you restore onto a replacement host, remove any stale SQLite sidecars before
starting the service:

```bash
sudo rm -f /var/lib/rustc-perf/julia.db-wal /var/lib/rustc-perf/julia.db-shm
```

## Verification Checklist

After the first full apply, verify all of these conditions:

1. `terraform output -raw site_url` returns the stable HTTPS URL.
2. The site opens successfully in a browser over HTTPS.
3. If `site_hostname` is set, the DNS `A` record resolves to `site_public_ip`.
4. Caddy is listening on ports `80` and `443` on the instance.
5. `curl -I http://127.0.0.1:2346/` succeeds on the instance.
6. `/var/lib/rustc-perf/julia.db` exists and is non-empty.
7. `/var/lib/rustc-perf/.state/` exists and contains the checkpoint files.
8. The S3 backup bucket contains at least one uploaded archive.
9. A deploy (`terraform apply` with a new digest) replaces the instance, and the new host comes back with its data and certificate intact.

## Day-2 Operations

Useful checks:

```bash
cd infra/terraform
terraform output -raw site_url
terraform output -raw site_public_ip
terraform output -raw backup_bucket_name
```

On the instance:

```bash
sudo systemctl status rustc-perf-site
sudo systemctl status rustc-perf-caddy
sudo journalctl -u rustc-perf-site -n 100 --no-pager
sudo journalctl -u rustc-perf-caddy -n 100 --no-pager
sudo docker ps
sudo docker logs rustc-perf-site --tail 100
sudo docker logs rustc-perf-caddy --tail 100
curl -I http://127.0.0.1:2346/
curl -I http://127.0.0.1/
sudo ss -ltnp | grep -E ':(80|443|2346)\b'
ls -lh /var/lib/rustc-perf
```

Useful interpretations:

- `rustc-perf-caddy` logs are the first place to look for certificate issuance, listener startup, and proxy errors.
- Use `sudo journalctl -u rustc-perf-caddy -f` when you want to watch certificate issuance or live proxy failures in real time.
- `rustc-perf-site` logs are the first place to look for Julia precompile progress and app startup failures.
- A temporary `502 Bad Gateway` from Caddy during first boot usually means the backend is still warming up.
- `rustc-perf-site` is not truly ready until `curl -I http://127.0.0.1:2346/` returns a normal HTTP response.

## Operational Rules

- Do not use mutable image tags for production deploys.
- Do not change `site_container_image` in Terraform for routine rollouts.
- Share and bookmark the stable HTTPS hostname (the `site_url` output), not the raw Elastic IP.
- Test a real restore from S3 before treating the backup path as production-ready.
- Use Session Manager for operator access; this stack does not provision SSH ingress.
- Expect first-start Julia precompilation to take a couple of minutes on a fresh host or fresh image and do not treat a short-lived Caddy `502` during that window as a proxy bug.
- Use this document as the operator checklist for first bring-up and deploys.