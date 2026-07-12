# Julia Compiler Performance Monitoring & Benchmarking

This repository contains the code and runtime assets for Julia compiler
performance monitoring.

Key pieces:

- [site](./site): the Rust web UI and HTTP API.
- [src](./src): the Julia orchestrator that ingests upstream data and keeps the
	site process running.
- [run.jl](./run.jl): the production entrypoint, which runs the orchestrator in
	install mode.

## Container Image

The repository root [Dockerfile](./Dockerfile) builds the full production image.
It compiles the embedded `prod_site` Rust binary, installs the Julia project,
and starts the app through `run.jl --install`.

The production Terraform path runs that image as an unprivileged service user
with `no-new-privileges` and no ambient Linux capabilities. The runtime data
directory on the host is owned by that fixed UID/GID automatically during
instance bootstrap.

The container expects persistent runtime data under one directory:

- `julia.db`: the SQLite database.
- `.state/`: the checkpoint files used by the Julia orchestrator.

Build the image locally with:

```bash
docker build -t rustc-perf:latest .
```

Run it locally against an existing database directory with:

```bash
docker run --rm -p 2346:2346 \
	-v "$PWD/runtime-data:/var/lib/rustc-perf" \
	rustc-perf:latest
```

The first start requires `runtime-data/julia.db` and both checkpoint files
(`runtime-data/.state/julia_last_processed_commit.txt` and
`runtime-data/.state/reports_last_processed_commit.txt`) to already exist —
in production all of them are restored from the S3 backup. The orchestrator
fails loudly if any of them is missing rather than guessing a starting point.

## Public Database Queries

Production exposes the live SQLite database through a read-only Datasette UI
and API at `/db/`. It supports schema browsing, `SELECT` queries, and
JSON/CSV exports; it does not expose a SQLite network port or database-file
downloads. Public queries are limited to one second and 1,000 rows, and may
briefly observe either side of an ingestion transaction.

To publish and deploy a new version to production, run
[infra/terraform/deploy.sh](./infra/terraform/deploy.sh): it builds the image
for `linux/amd64`, pushes it to ECR, takes a pre-deploy backup, and replaces
the instance with the new digest-pinned image.

Additional documentation on running and setting up the frontend and backend can
be found in the `README` files in the `collector` and `site` directories.

For the step-by-step production bring-up checklist, including deploys, backups,
and restore operations, see
[docs/production-deployment.md](./docs/production-deployment.md).

## License
The code of this repository is licensed under the `MIT` license, managed by the [`Reuse Specification`](REUSE.toml).
