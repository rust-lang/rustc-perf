# This Dockerfile is composed of two steps: the first one builds the release
# binary, and then the binary is copied inside another, empty image.

#################
#  Build image  #
#################

FROM ubuntu:22.04 as build

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

COPY . .
RUN bash -c 'source $HOME/.cargo/env && cargo test --release --all'
RUN bash -c 'source $HOME/.cargo/env && cargo build --release'

##################
#  Output image  #
##################

FROM ubuntu:22.04 AS binary

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
    ca-certificates

RUN mkdir -p /opt/triagebot

COPY --from=build /target/release/triagebot /usr/local/bin/
COPY templates /opt/triagebot/templates
WORKDIR /opt/triagebot
ENV PORT=80
CMD triagebot
