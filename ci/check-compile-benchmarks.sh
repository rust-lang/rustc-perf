#!/bin/bash

set -eE -x;

bash -c "while true; do sleep 30; echo \$(date) - running ...; done" &
PING_LOOP_PID=$!
trap 'kill $PING_LOOP_PID' ERR 1 2 3 6

BACKENDS=${BACKENDS:-Llvm}

# Install a toolchain.
RUST_BACKTRACE=1 RUST_LOG=raw_cargo_messages=trace,collector=debug,rust_sysroot=debug \
    bindir=`cargo run -p collector --bin collector install_next --backends ${BACKENDS}`

# Do some benchmarking.
RUST_BACKTRACE=1 \
    RUST_LIB_BACKTRACE=0 \
    CARGO_LOG=cargo::core::compiler::fingerprint=info \
    RUST_LOG=raw_cargo_messages=trace,collector=debug,rust_sysroot=debug \
    cargo run -p collector --bin collector -- \
    bench_local $bindir/rustc \
        --id Test \
        --profiles $PROFILES \
        --cargo $bindir/cargo \
        --scenarios All \
        --backends $BACKENDS \
        --rustdoc $bindir/rustdoc \
        $BENCH_INCLUDE_EXCLUDE_OPTS

kill $PING_LOOP_PID
exit 0
