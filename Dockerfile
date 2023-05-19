FROM node:18 as frontend

COPY ./site/frontend ./site/frontend
RUN cd site/frontend && npm ci
RUN cd site/frontend && npm run check
RUN cd site/frontend && npm run build

FROM ubuntu:20.04 as base

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

RUN apt-get update -y && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
      g++ \
      curl \
      ca-certificates \
      libc6-dev \
      make \
      libssl-dev \
      pkg-config \
      git \
      cmake \
      zlib1g-dev

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- \
    --default-toolchain stable --profile minimal -y

RUN bash -c 'source $HOME/.cargo/env && cargo install cargo-chef'

FROM base AS planner
COPY . .
RUN bash -c 'source $HOME/.cargo/env && cargo chef prepare --recipe-path recipe.json'

FROM base as build
COPY --from=planner recipe.json recipe.json

RUN bash -c 'source $HOME/.cargo/env && cargo chef cook --release --recipe-path recipe.json'

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./collector ./collector
COPY ./database ./database
COPY ./intern ./intern
COPY ./site ./site
COPY --from=frontend ./site/frontend/dist ./site/frontend/dist

RUN bash -c 'source $HOME/.cargo/env && cargo build --release -p site'
RUN bash -c 'source $HOME/.cargo/env && cargo build --release --bin postgres-to-sqlite'

FROM ubuntu:20.04 as binary

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
    ca-certificates \
    git

COPY --from=build /target/release/postgres-to-sqlite /usr/local/bin/rustc-perf-postgres-to-sqlite
COPY --from=build /target/release/site /usr/local/bin/rustc-perf-site

CMD rustc-perf-site
