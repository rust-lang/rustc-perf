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

The first start requires `runtime-data/julia.db` to already exist. If
`runtime-data/.state` is missing, the image derives the Julia master checkpoint
from the existing database and seeds the remaining checkpoint files
automatically.

To publish the image:

```bash
docker build -t rustc-perf:latest .
docker tag rustc-perf:latest <registry>/rustc-perf:latest
docker push <registry>/rustc-perf:latest
```

Pin the production deploy to the pushed digest rather than a mutable tag, for
example `<registry>/rustc-perf@sha256:...`, before updating Terraform.

After the first production apply, day-to-day deploys should move that digest
forward by updating the SSM image parameter from GitHub Actions and restarting
the existing host in place. That keeps `julia.db` and `.state/` on disk instead
of turning every application rollout into an EC2 replacement.

For now, the Terraform stack exposes the site through the ALB DNS name over
plain HTTP so it can deploy without CloudFront account verification.

Additional documentation on running and setting up the frontend and backend can
be found in the `README` files in the `collector` and `site` directories.

For the step-by-step production bring-up checklist, including backups, GitHub
Actions, HTTPS, and restore operations, see
[docs/production-deployment.md](./docs/production-deployment.md).

## License
The code of this repository is licensed under the `MIT` license, managed by the [`Reuse Specification`](REUSE.toml).

