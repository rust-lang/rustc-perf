# Production Deployment Runbook

Source of truth for bootstrapping the stack, deploying, debugging, backups,
and restore. Infrastructure details live in
[infra/terraform/README.md](../infra/terraform/README.md).

## The Model

- One EC2 instance runs the site container. Caddy on the same host terminates
  HTTP/HTTPS behind an Elastic IP, with an optional external DNS `A` record.
- The image digest and all host configuration live in `user_data`, so every
  deploy replaces the instance and the running host never drifts from the code.
- On boot the new host restores `julia.db`, `.state/`, and Caddy's certificate
  store from the latest S3 backup. The daily backup timer alone bounds a
  replacement's data loss to about a day; the pre-deploy backup in `deploy.sh`
  shrinks that to the few minutes before the apply.
- The runtime environment is fixed and non-secret
  (`/etc/rustc-perf-site.host.env` on the host). If the app ever needs a secret
  (for example a GitHub token to avoid rate limits), add it as an SSM
  SecureString and wire it into that env file and the instance IAM policy.

## First-Time Bootstrap

You need AWS credentials that can create IAM roles and a VPC; see the
credentials note in [infra/terraform/README.md](../infra/terraform/README.md)
if you authenticate with `aws login`.

1. Fill in `infra/terraform/terraform.tfvars`:

   ```
   aws_region          = "us-east-1"
   name_prefix         = "rustc-perf"
   instance_type       = "t3a.medium"
   root_volume_size_gb = 100
   site_hostname       = "rustserver.perf.julialang.org"
   ```

2. Create the image repository and backup bucket first, so the image push and
   the database seed have somewhere to go:

   ```bash
   cd infra/terraform
   terraform init
   terraform apply -target=aws_ecr_repository.site -target=aws_s3_bucket.backups
   ```

3. Seed the initial database — an archive containing `julia.db` and `.state/`
   (with both checkpoint files), in the layout a backup produces:

   ```bash
   aws s3 cp latest.tar.gz "s3://$(terraform output -raw backup_bucket_name)/runtime/latest.tar.gz"
   ```

4. Build, push, and apply:

   ```bash
   SKIP_BACKUP=1 ./deploy.sh   # no instance exists yet, so nothing to back up
   ```

5. Point DNS at the instance: create an `A` record for `site_hostname` →
   `terraform output -raw site_public_ip`. Caddy obtains the Let's Encrypt
   certificate automatically once the name resolves. Until then (or if
   `site_hostname` is unset) the site serves plain HTTP on the Elastic IP.

6. Run the post-bootstrap checklist at the bottom of this document.

## Deploying A New Version

```bash
cd infra/terraform
./deploy.sh
```

The script starts a backup on the instance, builds and pushes the image while
the backup runs, waits for the backup to succeed, then runs `terraform apply`
with the new digest — which replaces the instance. Terraform prompts before
replacing anything. Pass `terraform apply` flags through
(`./deploy.sh -auto-approve`), or set `SKIP_BACKUP=1` to skip the backup and
accept rolling back to the last daily one.

Host-configuration changes deploy the same way — edit the code and run
`./deploy.sh`. A bare `terraform apply` fails the image-digest precondition on
purpose: only `deploy.sh` supplies `-var site_container_image=...`, so every
replacement ships an image that was just built and pushed.

Expect a couple of minutes of downtime while the new instance boots, pulls the
image, and restores the database from S3. Julia precompilation is baked into
the image, so it does not add to this.

## When Something Goes Wrong

Log on with Session Manager and read the site unit's journal — that is the
right first move for almost any problem:

```bash
INSTANCE_ID="$(cd infra/terraform && terraform output -raw instance_id)"
aws ssm start-session --target "$INSTANCE_ID"

sudo journalctl -u rustc-perf-site -n 200 --no-pager   # or -f to follow
```

The unit runs the container in the foreground, so the journal carries all app
output (restore, Julia orchestrator, site binary); `sudo docker logs
rustc-perf-site` shows the same stream. For certificate issuance and proxy
errors, read the Caddy unit instead:
`sudo journalctl -u rustc-perf-caddy -n 200 --no-pager`.

Quick state checks:

```bash
sudo systemctl status rustc-perf-site rustc-perf-caddy
curl -I http://127.0.0.1:2346/   # the backend itself
curl -I http://127.0.0.1/        # through Caddy
```

Things to know while reading:

- First start on a fresh host takes a minute or two — image pull plus the S3
  restore — during which the unit shows `active` and Caddy returns
  `502 Bad Gateway`. The service is ready when
  `curl -I http://127.0.0.1:2346/` returns a normal HTTP response — not merely
  when the unit is running. Julia precompilation is baked into the image; if
  the journal streams `Precompiling` output on start, the baked caches were
  rejected because `JULIA_CPU_TARGET` in the Dockerfile does not cover this
  host's CPU — fix that rather than waiting it out.
- If the restore finds no database and no usable backup (empty bucket,
  download failure), `rustc-perf-restore-if-empty` fails the unit on purpose
  and systemd keeps retrying instead of starting an empty site. Seed the
  backup bucket (or fix backups), then
  `sudo systemctl restart rustc-perf-site`.
- If the instance is missing from Session Manager, apply the stack (the IAM
  instance profile is what grants SSM), wait a minute or two, then check
  `aws ssm describe-instance-information`.

There is no SSH ingress; Session Manager is the only operator path.

## Backups

Terraform installs a daily `rustc-perf-backup.timer`. Each run takes an online
snapshot with SQLite's `.backup` API (no downtime; a 30s busy timeout rides
out the orchestrator's write transactions), stages it under
`/var/lib/rustc-perf-backup-staging` on the root volume, and uploads:

- a timestamped archive at `<prefix>/archive/<hostname>/<ts>.tar.gz`, expired
  by S3 lifecycle after 30 days;
- `<prefix>/latest.tar.gz` — an independent full copy (an S3 server-side copy,
  not a reference), which never expires. It is the restore source for a
  replaced host and survives arbitrarily long backup outages.

Each archive contains `julia.db`, `.state/`, `caddy-data/` (the TLS
certificate store, so a replaced host reuses its certificate instead of
re-requesting from Let's Encrypt), and a `metadata.txt` with timestamp,
hostname, and image reference.

Manual run and inspection:

```bash
sudo /usr/local/bin/rustc-perf-backup
sudo journalctl -u rustc-perf-backup.service -n 100 --no-pager
systemctl list-timers rustc-perf-backup.timer
aws s3 ls s3://<backup-bucket>/runtime/ --recursive
```

If a backup fails, check that journal and root-volume free space.

## Restore

Automatic: on every service start, `rustc-perf-restore-if-empty` seeds
`julia.db`, `.state/`, and the Caddy store from `latest.tar.gz` (falling back
to the newest timestamped archive) — but **only when no local database is
present**. An existing database is never overwritten, so this is safe on every
start.

Manual (restoring a specific, older archive over current data):

1. `sudo systemctl stop rustc-perf-site`
2. Download and unpack the archive; replace the contents of
   `/var/lib/rustc-perf` with it.
3. Fix ownership, permissions, and stale SQLite sidecars:

   ```bash
   sudo chown -R 10001:10001 /var/lib/rustc-perf
   sudo chmod 750 /var/lib/rustc-perf /var/lib/rustc-perf/.state
   sudo chmod 640 /var/lib/rustc-perf/julia.db
   sudo rm -f /var/lib/rustc-perf/julia.db-wal /var/lib/rustc-perf/julia.db-shm
   ```

4. `sudo systemctl start rustc-perf-site && sudo systemctl restart rustc-perf-caddy`
5. Verify with the checks in "When Something Goes Wrong".

Test a real restore from S3 once before treating the backup path as
production-ready.

## Post-Bootstrap Checklist

1. `terraform output -raw site_url` opens in a browser — over HTTPS if
   `site_hostname` is set and its `A` record resolves to `site_public_ip`.
2. On the instance: `curl -I http://127.0.0.1:2346/` succeeds,
   `/var/lib/rustc-perf/julia.db` is non-empty, and
   `/var/lib/rustc-perf/.state/` contains both checkpoint files.
3. The backup bucket has a timestamped archive after the first timer run (or
   run `sudo /usr/local/bin/rustc-perf-backup` by hand).
4. `./deploy.sh` replaces the instance and the new host comes back with its
   data and certificate intact.
