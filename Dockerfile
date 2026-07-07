FROM node:20-bookworm-slim AS frontend

WORKDIR /work

COPY ./site/frontend ./site/frontend

RUN cd site/frontend && npm ci
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

RUN cargo build --release -p site --bin site && \
    cp ./target/release/site /tmp/prod_site

FROM julia:1.12-bookworm AS runtime

ENV RUSTC_PERF_APP_ROOT=/app
ENV JULIA_BIN=/usr/local/julia/bin/julia
ENV JULIA_DEPOT_PATH=/usr/local/julia-depot
# Must match container_runtime_uid/gid in infra/terraform/main.tf, which owns
# the bind-mounted data directory on the host.
ENV RUSTC_PERF_RUNTIME_UID=10001
ENV RUSTC_PERF_RUNTIME_GID=10001
WORKDIR /app

RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/* && \
    groupadd --system --gid "$RUSTC_PERF_RUNTIME_GID" rustc-perf && \
    useradd --system --uid "$RUSTC_PERF_RUNTIME_UID" --gid "$RUSTC_PERF_RUNTIME_GID" --create-home --home-dir /home/rustc-perf rustc-perf

COPY ./Project.toml ./Project.toml
COPY ./Manifest.toml ./Manifest.toml
COPY ./run.jl ./run.jl
COPY ./src ./src
COPY ./docker-entrypoint.sh /usr/local/bin/rustc-perf-entrypoint
COPY --from=rust-build /tmp/prod_site ./prod_site

RUN install -d -m 755 "$JULIA_DEPOT_PATH" && \
    chmod 755 /usr/local/bin/rustc-perf-entrypoint && \
    "$JULIA_BIN" --project=/app -e 'using Pkg; Pkg.instantiate(); Pkg.precompile(); using Orchestrator' && \
    chown -R "$RUSTC_PERF_RUNTIME_UID:$RUSTC_PERF_RUNTIME_GID" /app "$JULIA_DEPOT_PATH"

ENTRYPOINT ["/usr/local/bin/rustc-perf-entrypoint"]
