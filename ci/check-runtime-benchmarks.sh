#!/bin/bash

set -eE -x;

bash -c "while true; do sleep 30; echo \$(date) - running ...; done" &
PING_LOOP_PID=$!
trap 'kill $PING_LOOP_PID' ERR 1 2 3 6

# Install a toolchain.
RUST_BACKTRACE=1 RUST_LOG=raw_cargo_messages=trace,collector=debug,rust_sysroot=debug \
    bindir=`cargo run -p collector --bin collector install_next`

# Do some benchmarking.
RUST_LOG=raw_cargo_messages=trace,collector=debug,rust_sysroot=debug \
    cargo run -p collector --bin collector -- \
    bench_runtime_local $bindir/rustc \
        --cargo $bindir/cargo \
        --iterations 1 \
        --id Test

kill $PING_LOOP_PID
exit 0
