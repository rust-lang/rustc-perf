#!/bin/bash

set -x;

bash -c "while true; do sleep 30; echo \$(date) - running ...; done" &
PING_LOOP_PID=$!
trap - ERR
RUST_BACKTRACE=1 RUST_LOG=collector=debug,rust_sysroot=debug \
    cargo run -p collector --bin collector -- \
    --db temporary.db \
    $COLLECTOR_ARGS \
    test_benchmarks;
code=$?
kill $PING_LOOP_PID
exit $code
