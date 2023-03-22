#!/bin/bash

set -eE -x;

bash -c "while true; do sleep 30; echo \$(date) - running ...; done" &
PING_LOOP_PID=$!
trap 'kill $PING_LOOP_PID' ERR 1 2 3 6

# Install a toolchain.
RUST_BACKTRACE=1 RUST_LOG=raw_cargo_messages=trace,collector=debug,rust_sysroot=debug \
    bindir=`cargo run -p collector --bin collector install_next`

cargo build -p collector --bin rustc-fake

#----------------------------------------------------------------------------
# Test the profilers
#----------------------------------------------------------------------------

# These are basic smoke tests that run the profiler and check that the output
# looks plausible, mostly done by grepping for a single line of output that has
# a specific form.

# TODO: self-profile.

# perf-record: untested because we get "The instructions:u event is not
# supported" on GitHub Actions when the code below is run. Maybe because the
# hardware is virtualized and performance counters aren't available?
#RUST_BACKTRACE=1 RUST_LOG=raw_cargo_messages=trace,collector=debug,rust_sysroot=debug \
#    cargo run -p collector --bin collector -- \
#    profile_local perf-record $bindir/rustc \
#        --id Test \
#        --profiles Check \
#        --cargo $bindir/cargo \
#        --include helloworld \
#        --scenarios Full
#test -f results/perf-Test-helloworld-Check-Full
#grep -q "PERFILE" results/perf-Test-helloworld-Check-Full

# oprofile: untested... it's not used much, and might have the same problems
# that `perf` has due to virtualized hardware.

# samply: untested because it has the same problems that `perf` has due to
# virtualized hardware.

# Cachegrind.
RUST_BACKTRACE=1 RUST_LOG=raw_cargo_messages=trace,collector=debug,rust_sysroot=debug \
    cargo run -p collector --bin collector -- \
    profile_local cachegrind $bindir/rustc \
        --id Test \
        --profiles Check \
        --cargo $bindir/cargo \
        --include helloworld \
        --scenarios Full
test -f results/cgout-Test-helloworld-Check-Full
grep -q "events: Ir" results/cgout-Test-helloworld-Check-Full
test -f results/cgann-Test-helloworld-Check-Full
grep -q "PROGRAM TOTALS" results/cgann-Test-helloworld-Check-Full
# Ensure the jemalloc file/function aggregation is working.
grep -q "<all-jemalloc-files>:<all-jemalloc-functions>" results/cgann-Test-helloworld-Check-Full

# Callgrind.
RUST_BACKTRACE=1 RUST_LOG=raw_cargo_messages=trace,collector=debug,rust_sysroot=debug \
    cargo run -p collector --bin collector -- \
    profile_local callgrind $bindir/rustc \
        --id Test \
        --profiles Check \
        --cargo $bindir/cargo \
        --include helloworld \
        --scenarios Full
test -f results/clgout-Test-helloworld-Check-Full
grep -q "creator: callgrind" results/clgout-Test-helloworld-Check-Full
test -f results/clgann-Test-helloworld-Check-Full
grep -q "Profile data file" results/clgann-Test-helloworld-Check-Full

# DHAT.
RUST_BACKTRACE=1 RUST_LOG=raw_cargo_messages=trace,collector=debug,rust_sysroot=debug \
    cargo run -p collector --bin collector -- \
    profile_local dhat $bindir/rustc \
        --id Test \
        --profiles Check \
        --cargo $bindir/cargo \
        --include helloworld \
        --scenarios Full
test -f results/dhout-Test-helloworld-Check-Full
grep -q "dhatFileVersion" results/dhout-Test-helloworld-Check-Full


# DHAT (copy mode).
# FIXME: CI currently runs Ubuntu 20.04 LTS, which has a Valgrind that is too
# old to run with `--mode=copy`. When CI is upgraded to 22.04, uncomment this.
RUST_BACKTRACE=1 RUST_LOG=raw_cargo_messages=trace,collector=debug,rust_sysroot=debug \
    cargo run -p collector --bin collector -- \
    profile_local dhat-copy $bindir/rustc \
        --id Test \
        --profiles Check \
        --cargo $bindir/cargo \
        --include helloworld \
        --scenarios Full
test -f results/dhcopy-Test-helloworld-Check-Full
grep -q "dhatFileVersion" results/dhcopy-Test-helloworld-Check-Full

# Massif.
RUST_BACKTRACE=1 RUST_LOG=raw_cargo_messages=trace,collector=debug,rust_sysroot=debug \
    cargo run -p collector --bin collector -- \
    profile_local massif $bindir/rustc \
        --id Test \
        --profiles Check \
        --cargo $bindir/cargo \
        --include helloworld \
        --scenarios Full
test -f results/msout-Test-helloworld-Check-Full
grep -q "snapshot=0" results/msout-Test-helloworld-Check-Full

# Bytehound.
# This is currently broken in CI, commenting out to fix CI for this.
# RUST_BACKTRACE=1 RUST_LOG=raw_cargo_messages=trace,collector=debug,rust_sysroot=debug \
#     cargo run -p collector --bin collector -- \
#     profile_local bytehound $bindir/rustc \
#         --id Test \
#         --profiles Check \
#         --cargo $bindir/cargo \
#         --include helloworld \
#         --scenarios Full
# test -f results/bhout-Test-helloworld-Check-Full

# eprintln. The output file is empty because a vanilla rustc doesn't print
# anything to stderr.
RUST_BACKTRACE=1 RUST_LOG=raw_cargo_messages=trace,collector=debug,rust_sysroot=debug \
    cargo run -p collector --bin collector -- \
    profile_local eprintln $bindir/rustc \
        --id Test \
        --profiles Check \
        --cargo $bindir/cargo \
        --include helloworld \
        --scenarios Full
test   -f results/eprintln-Test-helloworld-Check-Full
test ! -s results/eprintln-Test-helloworld-Check-Full

# llvm-lines. `Debug` not `Check` because it doesn't support `Check` profiles.
# Including both `helloworld` and `regex-1.5.5` benchmarks, as they exercise the
# zero dependency and the greater than zero dependency cases, respectively, the
# latter of which has broken before.
RUST_BACKTRACE=1 RUST_LOG=raw_cargo_messages=trace,collector=debug,rust_sysroot=debug \
    cargo run -p collector --bin collector -- \
    profile_local llvm-lines $bindir/rustc \
        --id Test \
        --profiles Debug \
        --cargo $bindir/cargo \
        --include helloworld,regex-1.5.5 \
        --scenarios Full
test -f results/ll-Test-helloworld-Debug-Full
grep -q "Lines.*Copies.*Function name" results/ll-Test-helloworld-Debug-Full
test -f results/ll-Test-regex-1.5.5-Debug-Full
grep -q "Lines.*Copies.*Function name" results/ll-Test-regex-1.5.5-Debug-Full

# llvm-ir. `Debug` not `Check` because it works better that way.
RUST_BACKTRACE=1 RUST_LOG=raw_cargo_messages=trace,collector=debug,rust_sysroot=debug \
    cargo run -p collector --bin collector -- \
    profile_local llvm-ir $bindir/rustc \
        --id Test \
        --profiles Debug \
        --cargo $bindir/cargo \
        --include helloworld \
        --scenarios Full
test -f results/llir-Test-helloworld-Debug-Full
grep -q "; ModuleID" results/llir-Test-helloworld-Debug-Full

# mono-items. `Debug` not `Check` because it works better that way.
RUST_BACKTRACE=1 RUST_LOG=raw_cargo_messages=trace,collector=debug,rust_sysroot=debug \
    cargo run -p collector --bin collector -- \
    profile_local mono-items $bindir/rustc \
        --id Test \
        --profiles Debug \
        --cargo $bindir/cargo \
        --include helloworld \
        --scenarios Full
test -f results/mono-items-Test-helloworld-Debug-Full/raw
grep -q "MONO_ITEM" results/mono-items-Test-helloworld-Debug-Full/raw

# dep-graph. `IncrFull` not `Full` because it doesn't work with `Full`.
RUST_BACKTRACE=1 RUST_LOG=raw_cargo_messages=trace,collector=debug,rust_sysroot=debug \
    cargo run -p collector --bin collector -- \
    profile_local dep-graph $bindir/rustc \
        --id Test \
        --profiles Check \
        --cargo $bindir/cargo \
        --include helloworld \
        --scenarios IncrFull
test -f results/dep-graph-Test-helloworld-Check-IncrFull.txt
grep -q "hir_owner" results/dep-graph-Test-helloworld-Check-IncrFull.txt

#----------------------------------------------------------------------------
# Test option handling
#----------------------------------------------------------------------------

# With `--profiles` unspecified, `Check`/`Debug`/`Opt` files must be present,
# and `Doc` files must not be present.
RUST_BACKTRACE=1 RUST_LOG=raw_cargo_messages=trace,collector=debug,rust_sysroot=debug \
    cargo run -p collector --bin collector -- \
    profile_local eprintln $bindir/rustc \
        --id Builds1 \
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

# With `--profiles Doc` specified, `Check`/`Debug`/`Opt` files must not be
# present, and `Doc` files must be present (but not for incremental runs).
RUST_BACKTRACE=1 RUST_LOG=raw_cargo_messages=trace,collector=debug,rust_sysroot=debug \
    cargo run -p collector --bin collector -- \
    profile_local eprintln $bindir/rustc \
        --id Builds2 \
        --profiles Doc \
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

# With `--scenarios IncrUnchanged` specified, `IncrFull` and `IncrUnchanged`
# files must be present.
RUST_BACKTRACE=1 RUST_LOG=raw_cargo_messages=trace,collector=debug,rust_sysroot=debug \
    cargo run -p collector --bin collector -- \
    profile_local eprintln $bindir/rustc \
        --id Runs1 \
        --profiles Check \
        --cargo $bindir/cargo \
        --include helloworld \
        --scenarios IncrUnchanged
test ! -e results/eprintln-Runs1-helloworld-Check-Full
test   -f results/eprintln-Runs1-helloworld-Check-IncrFull
test   -f results/eprintln-Runs1-helloworld-Check-IncrUnchanged
test ! -e results/eprintln-Runs1-helloworld-Check-IncrPatched0

# With `--scenarios IncrPatched` specified, `IncrFull` and `IncrPatched0` files
# must be present.
RUST_BACKTRACE=1 RUST_LOG=raw_cargo_messages=trace,collector=debug,rust_sysroot=debug \
    cargo run -p collector --bin collector -- \
    profile_local eprintln $bindir/rustc \
        --id Runs2 \
        --profiles Check \
        --cargo $bindir/cargo \
        --include helloworld \
        --scenarios IncrPatched
test ! -e results/eprintln-Runs2-helloworld-Check-Full
test   -f results/eprintln-Runs2-helloworld-Check-IncrFull
test ! -e results/eprintln-Runs2-helloworld-Check-IncrUnchanged
test   -f results/eprintln-Runs2-helloworld-Check-IncrPatched0

kill $PING_LOOP_PID
exit 0
