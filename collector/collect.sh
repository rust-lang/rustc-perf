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
        cargo +nightly build --release -p collector

        # Install measureme tooling
        cargo install --git https://github.com/rust-lang/measureme --branch stable flamegraph crox summarize

        touch todo-artifacts
        for x in $(cat todo-artifacts) ; do
                echo "Benching $x from todo-artifacts"
                target/release/collector bench_published $x --db $DATABASE;
        done
        rm todo-artifacts
        touch todo-artifacts

        target/release/collector bench_next $SITE_URL --self-profile --db $DATABASE;
        echo finished run at `date`;
done
