# Ambient Occlusion Benchmark

> Originally written by Syoyo Fujita: https://github.com/syoyo/aobench

`aoench` is a small ambient occlusion renderer for benchmarking realworld
floating point performance in various languages.

![image_vector_par](https://user-images.githubusercontent.com/904614/41043073-653aa5be-69a3-11e8-8a9d-007def8516cc.png)

## Instructions


To run it with the default target options (replace `${NAME}` with an algorithm name):

```
> cargo run --release -- 800 600 --algo ${NAME}
```

Use `RUSTFLAGS` to set the target CPU, for example:

```
> RUSTFLAGS="-C target-cpu=native" cargo run --release -- 800 600 --algo ${NAME}
```

To build and run the Intel SPMD Program Compiler (ISPC) version:

```
> cd volta && make
> ao 800 600
```

You can download ISPC for free for all major architectures from [the ISPC website](https://ispc.github.io/downloads.html).

## Results

On a dual core AVX1 i5 @1.8 GHz:

| 800 x 600    | time [ms] <br> Rust | speedup [-] <br> Rust-vs-Rust | time [ms] <br> ISPC 1.9.2 | speedup [-] <br> ISPC-vs-Rust |
|--------------|---------------------|-------------------------------|---------------------------|-------------------------------|
| `scalar`     |                6266 |                          1.0x |                      4976 | clang7: 1.3x                  |
| `vector`     |                2369 |                          2.6x |                      1157 | 2.0x                          |
| `scalar_par` |                2443 |                          2.5x |                         - | -                             |
| `vector_par` |                 983 |                          6.4x |                       454 | 2.2x                          |

On a 28 core Xeon E5-2690 v4 @ 2.60GHz:

| 800 x 600    | time [ms] <br> Rust | speedup [-] <br> Rust-vs-Rust | time [ms] <br> ISPC 1.9.2 | speedup [-] <br> ISPC-vs-Rust |
|--------------|---------------------|-------------------------------|---------------------------|-------------------------------|
| `scalar`     |                3234 |                          1.0x |                      3536 | clang7: 0.9x                  |
| `vector`     |                1096 |                          3.0x |                       525 | 2.1x                          |
| `scalar_par` |                 132 |                         24.5x |                         - | -                             |
| `vector_par` |                  76 |                         42.6x |                        20 | 3.8x                          |

| 4096 x 4096  | time [ms] <br> Rust | speedup [-] <br> Rust-vs-Rust | time [ms] <br> ISPC 1.9.2 | speedup [-] <br> ISPC-vs-Rust |
|--------------|---------------------|-------------------------------|---------------------------|-------------------------------|
| `scalar`     |              116121 |                          1.0x |                    127161 | clang7: 0.9x                  |
| `vector`     |               40076 |                          2.9x |                     19707 | 2.0x                          |
| `scalar_par` |                3273 |                         35.5x |                         - | -                             |
| `vector_par` |                1398 |                         83.1x |                       644 | 2.2x                          |

## Overview

There are 4 main pieces in the `aobench` benchmark:

* ray-plane intersection algorithm: [source](https://github.com/rust-lang-nursery/packed_simd/tree/master/examples/aobench/src/intersection/ray_plane.rs)
* ray-sphere intersection algorithm: [source](https://github.com/rust-lang-nursery/packed_simd/tree/master/examples/aobench/src/intersection/ray_sphere.rs)
* ambient occlusion algorithm: [source](https://github.com/rust-lang-nursery/packed_simd/tree/master/examples/aobench/src/ambient_occlusion.rs)
* ray-casting the pixels:
  * scalar serial: [source](https://github.com/rust-lang-nursery/packed_simd/tree/master/examples/aobench/src/scalar.rs)
  * scalar parallel: [source](https://github.com/rust-lang-nursery/packed_simd/tree/master/examples/aobench/src/scalar_parallel.rs)
  * vector serial: [source](https://github.com/rust-lang-nursery/packed_simd/tree/master/examples/aobench/src/vector.rs)
  * vector parallel: [source](https://github.com/rust-lang-nursery/packed_simd/tree/master/examples/aobench/src/vector_parallel.rs)

The scalar and vectorized implementations of the intersection and ao algorithms
are in the same file so that they can be easily compared.

As a comparison, the ISPC sources of the same benchmark are [here](https://github.com/ispc/ispc/tree/master/examples/aobench).
