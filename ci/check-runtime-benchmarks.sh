#!/bin/bash

set -eE -x;

bash -c "while true; do sleep 30; echo \$(date) - running ...; done" &
PING_LOOP_PID=$!
trap 'kill $PING_LOOP_PID' ERR 1 2 3 6

# Check if the runtime benchmarks can be compiled.
# Once we can actually run the benchmarks on CI, we will also execute them here.
# Currently it is not possible because of `perf` permission issues when gathering perf. counters.
cd collector/runtime-benchmarks
cargo check

kill $PING_LOOP_PID
exit 0
