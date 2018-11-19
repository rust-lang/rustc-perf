# N-Body

This is the [`n-body` benchmark from the benchmarksgame][bg]. It models the orbits
of Jovian planets, using the same simple symplectic-integrator.

## Usage

It takes two arguments in this order:

* `n`: the number of iterations to perform
* (optional) `algorithm`: the algorithm to use - defaults to the fastest one.
  * `0`: scalar algorithm
  * `1`: SIMD algorithm

## Implementation

There are three kernels, two of which are only run twice independently of the
number of iterations (`offset_momentum` and `energy`). The `advance` kernel is
run once per iterations and uses 100% of the running time.

[bg]: https://benchmarksgame-team.pages.debian.net/benchmarksgame/description/nbody.html#nbody
