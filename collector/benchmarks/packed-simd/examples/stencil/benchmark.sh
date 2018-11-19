#!/usr/bin/env bash
#
# Runs aobench benchmarks

set -ex

if [[ ${NORUN} != 1 ]]; then
    hash hyperfine 2>/dev/null || { echo >&2 "hyperfine is not in PATH."; exit 1; }
fi

algs=("0" "1" "2")
if echo "$FEATURES" | grep -q "ispc"; then
    hash ispc 2>/dev/null || { echo >&2 "ispc is not in PATH."; exit 1; }
    algs+=( "3" "4" )
fi

RUSTFLAGS="-C target-cpu=native ${RUSTFLAGS}" \
         cargo build --release --no-default-features \
         --features="${FEATURES}"

if [[ "${VERIFY}" == "1" ]]; then
    RUSTFLAGS="-C target-cpu=native ${RUSTFLAGS}" \
             cargo test --release --no-default-features \
             --features="${FEATURES}"
fi

if [[ "${NORUN}" == "1" ]]; then
    exit 0
fi

for alg in "${algs[@]}"
do
    hyperfine "target/release/stencil ${alg}"
done
