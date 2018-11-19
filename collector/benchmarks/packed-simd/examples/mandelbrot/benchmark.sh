#!/usr/bin/env bash
#
# Runs mandelbrot benchmarks

set -ex

WIDTH=800
HEIGHT=800

if [[ ${NORUN} != 1 ]]; then
    hash hyperfine 2>/dev/null || { echo >&2 "hyperfine is not in PATH."; exit 1; }
fi

if echo "$FEATURES" | grep -q "ispc"; then
    hash ispc 2>/dev/null || { echo >&2 "ispc is not in PATH."; exit 1; }
fi

RUSTFLAGS="-C target-cpu=native ${RUSTFLAGS}" \
         cargo build --release --features="${FEATURES}"

if [[ "${VERIFY}" == "1" ]]; then
    RUSTFLAGS="-C target-cpu=native ${RUSTFLAGS}" \
             cargo test --release --features="${FEATURES}"
fi

if [[ "${NORUN}" == "1" ]]; then
    exit 0
fi

hyperfine "target/release/mandelbrot ${WIDTH} ${HEIGHT} --algo scalar"
hyperfine "target/release/mandelbrot ${WIDTH} ${HEIGHT} --algo simd"

if echo "$FEATURES" | grep -q "ispc"; then
    hyperfine "target/release/mandelbrot ${WIDTH} ${HEIGHT} --algo ispc"
fi
