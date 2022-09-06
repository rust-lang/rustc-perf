#!/bin/bash

# This script expects SITE_URL and DATABASE to be defined in the environment

set -u -o pipefail

echo "Running collector targeting $SITE_URL"

export RUST_LOG=collector=trace,collector::sysroot=debug
export PATH="/home/collector/.cargo/bin:$PATH"

while : ; do
        # Update and rebuild the collector.
        git pull
        git reset --hard @{upstream}

        # Make sure we have a recent build, so that we can successfully build
        # the collector.
        rustup update
        cargo +nightly build --release -p collector

        # Install measureme tooling
        cargo install --git https://github.com/rust-lang/measureme --branch stable flamegraph crox summarize

        target/release/collector bench_next $SITE_URL --self-profile --bench-rustc --db $DATABASE;
        echo finished run at `date`;

        # Wait a little bit before the next run starts.
        sleep 120
done
