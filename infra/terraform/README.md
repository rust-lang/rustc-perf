# AWS Terraform Deployment

This directory provisions a deliberately small deployment of the site on AWS.
This file documents the infrastructure's shape and the decisions behind it;
the operational runbook (first bring-up, deploys, debugging, backups, restore)
is [docs/production-deployment.md](../../docs/production-deployment.md).

What the stack creates:

- One EC2 instance running the production container in Docker, in a minimal
  dedicated VPC: one public subnet, an internet gateway, and a security group
  exposing only ports 80 and 443.
- One Elastic IP attached to the instance. An optional external DNS `A` record
  (`site_hostname`) points at it.
- Caddy on the instance for HTTP and automatic Let's Encrypt HTTPS.
- One encrypted gp3 root volume holding the OS, Docker data, the SQLite
  database (`/var/lib/rustc-perf/julia.db`), and the orchestrator checkpoints
  (`/var/lib/rustc-perf/.state`).
- One ECR repository for the site image, with immutable tags so a push cannot
  silently replace a referenced image.
- One private, encrypted S3 bucket for runtime backups.
- An instance profile granting Session Manager access (the only operator path;
  no SSH ingress) and the S3 backup permissions.

The container entrypoint runs [run.jl](../../run.jl) with `--install`: the
Julia orchestrator ingests fresh data continuously and runs the embedded
`prod_site` Rust binary.

## Why this shape

- SQLite wants a normal local filesystem, and one EC2 host is easy to
  understand, inspect, and roll back. EFS would be the wrong default.
- An Elastic IP plus Caddy keeps the public endpoint stable without Route53,
  ACM, CloudFront, or an ALB.
- Deploys are immutable: the image digest and every host setting live in
  `user_data` (`user_data_replace_on_change = true`), so `./deploy.sh`
  replaces the instance and the running machine never drifts from the code.
  Replacement is non-destructive — on boot the new host restores `julia.db`,
  `.state`, and Caddy's TLS certificates from the latest S3 backup, so no data
  is lost and no certificate is re-requested.
- Backups run on a daily timer using SQLite's online `.backup` (no downtime),
  and `deploy.sh` takes a fresh pre-deploy backup automatically.
- Hardening appropriate for a public deployment: IMDSv2 required, only 80/443
  exposed, the container runs as a fixed non-root UID/GID with
  `no-new-privileges` and no Linux capabilities, and the data directory is
  private to that user (`0750` directories, `0640` database).

## Configuration

`terraform.tfvars` is committed with the production values (region, instance
type, volume size, hostname); there is nothing to fill in. The remaining fixed
settings — paths, port, UID/GID, backup layout, retention — are locals at the
top of [main.tf](main.tf) and flow from there into the host env file, the
scripts, and `docker build` arguments, so each value has one home.

`site_container_image` is deliberately not in `terraform.tfvars`: `deploy.sh`
passes the freshly pushed digest as `-var site_container_image=...`, and an
instance precondition rejects anything not `@sha256`-pinned. A bare
`terraform apply` therefore fails by design — every instance replacement ships
an image that was just built and pushed.

## State

State lives in a local `terraform.tfstate`, which is fine for a single
operator. Losing it is recoverable (re-import the ~15 resources; the database
and certificates live in the S3 backup bucket, so no data is at risk). If more
than one person will ever apply, add a remote S3 backend with locking in
[versions.tf](versions.tf) and run `terraform init -migrate-state`.

## Prerequisites

1. Terraform 1.6+, Docker, and the AWS CLI.
2. AWS credentials that can create EC2, VPC, ECR, S3, and IAM (create-role)
   resources. Note that the PowerUserAccess permission set cannot manage IAM.

### Credentials note for `aws login`

The Terraform AWS provider cannot read credentials from the CLI's
browser-based `aws login` flow
([terraform-provider-aws#45316](https://github.com/hashicorp/terraform-provider-aws/issues/45316)).
If that is how you authenticate, bridge it in `~/.aws/config` and run
Terraform (and `deploy.sh`) with that profile:

```ini
[profile terraform]
credential_process = aws configure export-credentials --profile default --format process
region = us-east-1
```

```bash
AWS_PROFILE=terraform ./deploy.sh
```

## First deploy

Follow "First-Time Bootstrap" in
[docs/production-deployment.md](../../docs/production-deployment.md): a
targeted apply creates the ECR repository and backup bucket, you seed the
initial database into the bucket, and `SKIP_BACKUP=1 ./deploy.sh` does the
rest.

## Deploying a new version

```bash
./deploy.sh
```

It starts a pre-deploy backup on the instance, builds and pushes the image
(for `linux/amd64`) while the backup runs, then applies with the new digest —
which replaces the instance. Terraform prompts before replacing anything; pass
`terraform apply` flags through (`./deploy.sh -auto-approve`), or set
`SKIP_BACKUP=1` to skip the backup and accept rolling back to the last daily
one. Host-setting changes ship the same way: edit the code, run `./deploy.sh`.

Downtime is a couple of minutes, dominated by the new host pulling the image
and restoring the database from S3 — Julia precompilation is baked into the
image.

If something looks wrong afterwards, log on with Session Manager and read the
site journal — see "When Something Goes Wrong" in
[docs/production-deployment.md](../../docs/production-deployment.md).

## Backups and restore

A daily timer uploads archives of `julia.db`, `.state/`, and Caddy's
certificate store. Timestamped archives expire after 30 days;
`<prefix>/latest.tar.gz` — an independent full copy, not a reference — never
expires and is the automatic restore source for a replaced host. On service
start, `rustc-perf-restore-if-empty` seeds whatever is missing and never
overwrites an existing database. Details, manual backups, and the manual
restore procedure are in
[docs/production-deployment.md](../../docs/production-deployment.md).
