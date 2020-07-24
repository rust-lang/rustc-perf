#!/bin/bash
#
# This script only tests some of the profilers at the moment. More coverage
# would be nice.

set -eE -x;

bash -c "while true; do sleep 30; echo \$(date) - running ...; done" &
PING_LOOP_PID=$!
trap 'kill $PING_LOOP_PID' ERR 1 2 3 6

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

# Profile with llvm-lines.
RUST_BACKTRACE=1 RUST_LOG=collector_raw_cargo=trace,collector=debug,rust_sysroot=debug \
    cargo run -p collector --bin collector -- \
    profile_local llvm-lines $bindir/rustc Test \
        --builds Debug \
        --cargo $bindir/cargo \
        --include futures \
        --runs Full

# Check the output is present and looks something like it should.
test -f results/ll-Test-futures-Debug-Full
grep -q "Lines.*Copies.*Function name" results/ll-Test-futures-Debug-Full

kill $PING_LOOP_PID
exit 0
