#!/bin/bash
#
# This script only tests some of the profilers at the moment. More coverage
# would be nice.

set -eE -x;

bash -c "while true; do sleep 30; echo \$(date) - running ...; done" &
PING_LOOP_PID=$!
trap 'kill $PING_LOOP_PID' ERR 1 2 3 6

# Install a toolchain.
RUST_BACKTRACE=1 RUST_LOG=raw_cargo_messages=trace,collector=debug,rust_sysroot=debug \
    bindir=`cargo run -p collector --bin collector install_next`

#----------------------------------------------------------------------------
# Test the profilers
#----------------------------------------------------------------------------

# time-passes.
RUST_BACKTRACE=1 RUST_LOG=raw_cargo_messages=trace,collector=debug,rust_sysroot=debug \
    cargo run -p collector --bin collector -- \
    profile_local time-passes $bindir/rustc Test \
        --builds Check \
        --cargo $bindir/cargo \
        --include helloworld \
        --runs Full
test -f results/Ztp-Test-helloworld-Check-Full
grep -q "time:.*total" results/Ztp-Test-helloworld-Check-Full

# perf-record: TODO

# oprofile: TODO

# Cachegrind.
RUST_BACKTRACE=1 RUST_LOG=raw_cargo_messages=trace,collector=debug,rust_sysroot=debug \
    cargo run -p collector --bin collector -- \
    profile_local cachegrind $bindir/rustc Test \
        --builds Check \
        --cargo $bindir/cargo \
        --include helloworld \
        --runs Full
test -f results/cgout-Test-helloworld-Check-Full
grep -q "events: Ir" results/cgout-Test-helloworld-Check-Full
test -f results/cgann-Test-helloworld-Check-Full
grep -q "PROGRAM TOTALS" results/cgann-Test-helloworld-Check-Full

# Callgrind.
RUST_BACKTRACE=1 RUST_LOG=raw_cargo_messages=trace,collector=debug,rust_sysroot=debug \
    cargo run -p collector --bin collector -- \
    profile_local callgrind $bindir/rustc Test \
        --builds Check \
        --cargo $bindir/cargo \
        --include helloworld \
        --runs Full
test -f results/clgout-Test-helloworld-Check-Full
grep -q "creator: callgrind" results/clgout-Test-helloworld-Check-Full
test -f results/clgann-Test-helloworld-Check-Full
grep -q "Profile data file" results/clgann-Test-helloworld-Check-Full

# DHAT: needs Valgrind 3.15, but only Valgrind 3.13 is available on GitHub.

# Massif.
RUST_BACKTRACE=1 RUST_LOG=raw_cargo_messages=trace,collector=debug,rust_sysroot=debug \
    cargo run -p collector --bin collector -- \
    profile_local massif $bindir/rustc Test \
        --builds Check \
        --cargo $bindir/cargo \
        --include helloworld \
        --runs Full
test -f results/msout-Test-helloworld-Check-Full
grep -q "snapshot=0" results/msout-Test-helloworld-Check-Full

# eprintln. The output file is empty because a vanilla rustc doesn't print
# anything to stderr.
RUST_BACKTRACE=1 RUST_LOG=raw_cargo_messages=trace,collector=debug,rust_sysroot=debug \
    cargo run -p collector --bin collector -- \
    profile_local eprintln $bindir/rustc Test \
        --builds Check \
        --cargo $bindir/cargo \
        --include helloworld \
        --runs Full
test   -f results/eprintln-Test-helloworld-Check-Full
test ! -s results/eprintln-Test-helloworld-Check-Full

# llvm-lines. `Debug` not `Check` because it doesn't support `Check` builds.
RUST_BACKTRACE=1 RUST_LOG=raw_cargo_messages=trace,collector=debug,rust_sysroot=debug \
    cargo run -p collector --bin collector -- \
    profile_local llvm-lines $bindir/rustc Test \
        --builds Debug \
        --cargo $bindir/cargo \
        --include helloworld \
        --runs Full
test -f results/ll-Test-helloworld-Debug-Full
grep -q "Lines.*Copies.*Function name" results/ll-Test-helloworld-Debug-Full

#----------------------------------------------------------------------------
# Test option handling
#----------------------------------------------------------------------------

# With `--builds` unspecified, `Check`/`Debug`/`Opt` files must be present, and
# `Doc` files must not be present.
RUST_BACKTRACE=1 RUST_LOG=raw_cargo_messages=trace,collector=debug,rust_sysroot=debug \
    cargo run -p collector --bin collector -- \
    profile_local eprintln $bindir/rustc Builds1 \
        --cargo $bindir/cargo \
        --include helloworld
test   -f results/eprintln-Builds1-helloworld-Check-Full
test   -f results/eprintln-Builds1-helloworld-Check-IncrFull
test   -f results/eprintln-Builds1-helloworld-Check-IncrPatched0
test   -f results/eprintln-Builds1-helloworld-Check-IncrUnchanged
test   -f results/eprintln-Builds1-helloworld-Debug-Full
test   -f results/eprintln-Builds1-helloworld-Debug-IncrFull
test   -f results/eprintln-Builds1-helloworld-Debug-IncrPatched0
test   -f results/eprintln-Builds1-helloworld-Debug-IncrUnchanged
test   -f results/eprintln-Builds1-helloworld-Opt-Full
test   -f results/eprintln-Builds1-helloworld-Opt-IncrFull
test   -f results/eprintln-Builds1-helloworld-Opt-IncrPatched0
test   -f results/eprintln-Builds1-helloworld-Opt-IncrUnchanged
test ! -e results/eprintln-Builds1-helloworld-Doc-Full
test ! -e results/eprintln-Builds1-helloworld-Doc-IncrFull
test ! -e results/eprintln-Builds1-helloworld-Doc-IncrPatched0
test ! -e results/eprintln-Builds1-helloworld-Doc-IncrUnchanged

# With `--builds Doc` specified, `Check`/`Debug`/`Opt` files must not be
# present, and `Doc` files must be present (but not for incremental runs).
RUST_BACKTRACE=1 RUST_LOG=raw_cargo_messages=trace,collector=debug,rust_sysroot=debug \
    cargo run -p collector --bin collector -- \
    profile_local eprintln $bindir/rustc Builds2 \
	--builds Doc \
        --cargo $bindir/cargo \
        --include helloworld
test ! -e results/eprintln-Builds2-helloworld-Check-Full
test ! -e results/eprintln-Builds2-helloworld-Check-IncrFull
test ! -e results/eprintln-Builds2-helloworld-Check-IncrUnchanged
test ! -e results/eprintln-Builds2-helloworld-Check-IncrPatched0
test ! -e results/eprintln-Builds2-helloworld-Debug-Full
test ! -e results/eprintln-Builds2-helloworld-Debug-IncrFull
test ! -e results/eprintln-Builds2-helloworld-Debug-IncrUnchanged
test ! -e results/eprintln-Builds2-helloworld-Debug-IncrPatched0
test ! -e results/eprintln-Builds2-helloworld-Opt-Full
test ! -e results/eprintln-Builds2-helloworld-Opt-IncrFull
test ! -e results/eprintln-Builds2-helloworld-Opt-IncrUnchanged
test ! -e results/eprintln-Builds2-helloworld-Opt-IncrPatched0
test   -f results/eprintln-Builds2-helloworld-Doc-Full
test ! -f results/eprintln-Builds2-helloworld-Doc-IncrFull
test ! -f results/eprintln-Builds2-helloworld-Doc-IncrPatched0
test ! -f results/eprintln-Builds2-helloworld-Doc-IncrUnchanged

# With `--runs IncrUnchanged` specified, `IncrFull` and `IncrUnchanged` files
# must be present.
RUST_BACKTRACE=1 RUST_LOG=raw_cargo_messages=trace,collector=debug,rust_sysroot=debug \
    cargo run -p collector --bin collector -- \
    profile_local eprintln $bindir/rustc Runs1 \
	--builds Check \
        --cargo $bindir/cargo \
        --include helloworld \
        --runs IncrUnchanged
test ! -e results/eprintln-Runs1-helloworld-Check-Full
test   -f results/eprintln-Runs1-helloworld-Check-IncrFull
test   -f results/eprintln-Runs1-helloworld-Check-IncrUnchanged
test ! -e results/eprintln-Runs1-helloworld-Check-IncrPatched0

# With `--runs IncrPatched` specified, `IncrFull` and `IncrPatched0` files must
# be present.
RUST_BACKTRACE=1 RUST_LOG=raw_cargo_messages=trace,collector=debug,rust_sysroot=debug \
    cargo run -p collector --bin collector -- \
    profile_local eprintln $bindir/rustc Runs2 \
	--builds Check \
        --cargo $bindir/cargo \
        --include helloworld \
        --runs IncrPatched
test ! -e results/eprintln-Runs2-helloworld-Check-Full
test   -f results/eprintln-Runs2-helloworld-Check-IncrFull
test ! -e results/eprintln-Runs2-helloworld-Check-IncrUnchanged
test   -f results/eprintln-Runs2-helloworld-Check-IncrPatched0

kill $PING_LOOP_PID
exit 0
