# Spectral norm

This is the [`spectral-norm` benchmark from the benchmarksgame][bg]. 

## Background and description

MathWorld: ["Hundred-Dollar, Hundred-Digit Challenge Problems"](http://mathworld.wolfram.com/Hundred-DollarHundred-DigitChallengeProblems.html), [Challenge #3](http://mathworld.wolfram.com/SpectralNorm.html).

Each program should:

* calculate the spectral norm of an infinite matrix `A`, with entries `a11=1`,
  `a12=1/2`, `a21=1/3`, `a13=1/4`, `a22=1/5`, `a31=1/6`, etc.

* implement 4 separate functions / procedures / methods like the [C#
  program](https://benchmarksgame-team.pages.debian.net/benchmarksgame/program/spectralnorm-csharpcore-1.html)

## Usage

It takes two arguments in this order:

* `n`: the size of the matrix `A` (n-times-n)
* (optional) `algorithm`: the algorithm to use - defaults to the fastest one.
  * `0`: scalar algorithm
  * `1`: SIMD algorithm

[bg]: https://benchmarksgame-team.pages.debian.net/benchmarksgame/description/spectralnorm.html#spectralnorm
