#!/usr/bin/env bash
#
# Runs options_pricing benchmarks

set -ex

NUM_OPTIONS_BLACK_SCHOLES=10000000

if [[ ${NORUN} != 1 ]]; then
    hash hyperfine 2>/dev/null || { echo >&2 "hyperfine is not in PATH."; exit 1; }
fi

# Black-Scholes:
ALGS=("black_scholes_scalar" "black_scholes_simd" "black_scholes_simd_par")
if echo "$FEATURES" | grep -q "ispc"; then
    hash ispc 2>/dev/null || { echo >&2 "ispc is not in PATH."; exit 1; }
    ALGS+=("black_scholes_ispc" "black_scholes_ispc_tasks")
fi


RUSTFLAGS="-C target-cpu=native ${RUSTFLAGS}" \
         cargo build --release --features="${FEATURES}"

if [[ "${NORUN}" == "1" ]]; then
    exit 0
fi

#for alg in "${ALGS[@]}"
#do
#    hyperfine "target/release/options_pricing ${NUM_OPTIONS_BLACK_SCHOLES} ${alg}"
#done

# Binomial put:
ALGS=("binomial_put_scalar" "binomial_put_simd" "binomial_put_simd_par")
if echo "$FEATURES" | grep -q "ispc"; then
    ALGS+=("binomial_put_ispc" "binomial_put_ispc_tasks")
fi

NUM_OPTIONS_BINOMIAL_PUT=500000

for alg in "${ALGS[@]}"
do
    hyperfine "target/release/options_pricing ${NUM_OPTIONS_BINOMIAL_PUT} ${alg}"
done
