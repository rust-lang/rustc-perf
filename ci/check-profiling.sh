#!/bin/bash
#
# This script is basically just a smoke test. It only tests `eprintln`
# profiling because setting up the other profilers is something of a hassle.

set -e -x;

bash -c "while true; do sleep 30; echo \$(date) - running ...; done" &
PING_LOOP_PID=$!
trap 'kill $PING_LOOP_PID' ERR

# Install a toolchain.
RUST_BACKTRACE=1 RUST_LOG=collector_raw_cargo=trace,collector=debug,rust_sysroot=debug \
    bindir=`cargo run -p collector --bin collector install_next`

# Profile with eprintln.
RUST_BACKTRACE=1 RUST_LOG=collector_raw_cargo=trace,collector=debug,rust_sysroot=debug \
    cargo run -p collector --bin collector -- \
    profile_local eprintln $bindir/rustc Test \
        --cargo $bindir/cargo \
        --include helloworld \
        --runs Full

# Check that Check/Debug/Opt files are present, and that Doc files aren't
# present, becuase they're not done by default.
test -f results/eprintln-Test-helloworld-Check-Full
test -f results/eprintln-Test-helloworld-Debug-Full
test -f results/eprintln-Test-helloworld-Opt-Full
test ! -e results/eprintln-Test-helloworld-Doc-Full

kill $PING_LOOP_PID
exit 0
