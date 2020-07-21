#!/bin/bash

set -x;

bash -c "while true; do sleep 30; echo \$(date) - running ...; done" &
PING_LOOP_PID=$!
trap - ERR
RUST_BACKTRACE=1 RUST_LOG=collector_raw_cargo=trace,collector=debug,rust_sysroot=debug \
    bindir=`cargo run -p collector --bin collector install_next` \
    && \
RUST_BACKTRACE=1 RUST_LOG=collector_raw_cargo=trace,collector=debug,rust_sysroot=debug \
    cargo run -p collector --bin collector -- \
    bench_local $bindir/rustc Test \
        --builds Check,Doc \
        --cargo $bindir/cargo \
        --runs All \
        --rustdoc $bindir/rustdoc \
        $BENCH_INCLUDE_EXCLUDE_OPTS
code=$?
kill $PING_LOOP_PID
exit $code
