FROM node:20-bookworm-slim AS frontend

WORKDIR /work

# Dependencies install in a layer keyed only on the lockfile, so source-only
# changes reuse it instead of re-running npm ci.
COPY ./site/frontend/package.json ./site/frontend/package-lock.json ./site/frontend/
RUN cd site/frontend && npm ci

COPY ./site/frontend ./site/frontend
RUN cd site/frontend && npm run check
RUN cd site/frontend && npm run build

FROM rust:1-bookworm AS chef

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN cargo install cargo-chef --locked
WORKDIR /work

# cargo-chef splits the build so third-party dependencies are compiled in a
# layer keyed only on the recipe (i.e. the manifests); source-only changes
# reuse it instead of rebuilding every dependency.
FROM chef AS planner

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./collector ./collector
COPY ./database ./database
COPY ./intern ./intern
COPY ./site ./site
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS rust-build

COPY --from=planner /work/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./collector ./collector
COPY ./database ./database
COPY ./intern ./intern
COPY ./site ./site
COPY --from=frontend /work/site/frontend/dist ./site/frontend/dist

RUN cargo build --release -p site --bin site

FROM julia:1.12-bookworm AS runtime

ENV RUSTC_PERF_APP_ROOT=/app
ENV RUSTC_PERF_DATA_DIR=/var/lib/rustc-perf
ENV RUSTC_PERF_DB_FILENAME=julia.db
ENV JULIA_BIN=/usr/local/julia/bin/julia
ENV JULIA_DEPOT_PATH=/usr/local/julia-depot
# Precompile for the same portable CPU targets Julia's official binaries use,
# not the build machine's CPU. Otherwise the caches baked below are rejected
# on hosts with a different microarchitecture (e.g. older EC2 CPUs) and Julia
# re-precompiles everything on every container start.
ENV JULIA_CPU_TARGET="generic;sandybridge,-xsaveopt,clone_all;haswell,-rdrnd,base(1)"
# The UID/GID are owned by container_runtime_uid/gid in infra/terraform/main.tf
# and passed in by deploy.sh; these defaults only matter for images built by
# hand and match the Terraform values.
ARG RUSTC_PERF_RUNTIME_UID=10001
ARG RUSTC_PERF_RUNTIME_GID=10001
ENV RUSTC_PERF_RUNTIME_UID=${RUSTC_PERF_RUNTIME_UID}
ENV RUSTC_PERF_RUNTIME_GID=${RUSTC_PERF_RUNTIME_GID}
WORKDIR /app

RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/* && \
    groupadd --system --gid "$RUSTC_PERF_RUNTIME_GID" rustc-perf && \
    useradd --system --uid "$RUSTC_PERF_RUNTIME_UID" --gid "$RUSTC_PERF_RUNTIME_GID" --create-home --home-dir /home/rustc-perf rustc-perf && \
    install -d -m 755 -o "$RUSTC_PERF_RUNTIME_UID" -g "$RUSTC_PERF_RUNTIME_GID" "$JULIA_DEPOT_PATH" && \
    chown "$RUSTC_PERF_RUNTIME_UID:$RUSTC_PERF_RUNTIME_GID" /app

# Precompile as the runtime user so the depot is correctly owned as it is
# written, instead of a final chown -R that would duplicate the whole depot
# in an extra image layer.
USER rustc-perf

# Third-party dependencies precompile in a layer keyed only on the manifests;
# a stub module stands in for Orchestrator so app-code changes reuse it.
COPY --chown=rustc-perf:rustc-perf ./Project.toml ./Manifest.toml ./
RUN mkdir src && echo 'module Orchestrator end' > src/Orchestrator.jl && \
    "$JULIA_BIN" --project=/app -e 'using Pkg; Pkg.instantiate(); Pkg.precompile()' && \
    rm -r src

COPY --chown=rustc-perf:rustc-perf ./run.jl ./run.jl
COPY --chown=rustc-perf:rustc-perf ./src ./src
RUN "$JULIA_BIN" --project=/app -e 'using Pkg; Pkg.precompile(); using Orchestrator'

# The entrypoint and the site binary land last: a Rust-only change must not
# invalidate the Julia precompile layers above.
COPY --chmod=755 ./docker-entrypoint.sh /usr/local/bin/rustc-perf-entrypoint
COPY --from=rust-build /work/target/release/site ./prod_site

ENTRYPOINT ["/usr/local/bin/rustc-perf-entrypoint"]
