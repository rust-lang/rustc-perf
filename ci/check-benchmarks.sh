#!/bin/bash

set -x;

bash -c "while true; do sleep 30; echo \$(date) - running ...; done" &
PING_LOOP_PID=$!
trap - ERR
cargo build -p collector;
RUST_BACKTRACE=1 RUST_LOG=collector=trace,rust_sysroot=debug \
    cargo run -p collector --bin collector -- \
    --db temporary.db \
    --exclude servo,cargo,crates.io,packed-simd,sentry-cli,tuple-stress test_benchmarks;
code=$?
kill $PING_LOOP_PID
exit $code
