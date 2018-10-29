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

## Results

```
./benchmark.sh
```

On a dual core AVX1 i5 @1.8 GHz:

| 800 x 600    | time [ms] <br> Rust | speedup vs `scalar` [-] |
|--------------|---------------------|-------------------------|
| `scalar`     | 5884                | 1.0x                    |
| `scalar_par` | 2206                | 2.7x                    |
| `vector`     | 1458                | 4.0x                    |
| `vector_par` | 622                 | 9.5x                    |
| `tiled`      | 1328                | 4.4x                    |
| `tiled_par`  | 578                 | 10.2x                   |
| `ispc`       | 1158                | 5.1x                    |
| `ispc_tasks` | 567                 | 10.4x                   |

`tiled_par` is 1.02x slower than `ispc_tasks`.

On a 28 core Xeon CPU E5-2690 v4 @ 2.60GHz:

| 800 x 600    | time [ms] <br> Rust | speedup vs `scalar` [-] |
|--------------|---------------------|-------------------------|
| `scalar`     | 2981                | 1.0x                    |
| `scalar_par` | 163                 | 18.2x                   |
| `vector`     | 692                 | 4.3x                    |
| `vector_par` | 98                  | 30.4x                   |
| `tiled`      | 640                 | 4.7x                    |
| `tiled_par`  | 98                  | 30.4x                   |
| `ispc`       | 576                 | 5.2x                    |
| `ispc_tasks` | 150                 | 19.9x                   |

`tiled_par` is 1.53x faster than `ispc_tasks`.


On a 40 core Xeon Gold 6148 CPU @ 2.40GHz:

| 800 x 600    | time [ms] <br> Rust | speedup vs `scalar` [-] |
|--------------|---------------------|-------------------------|
| `scalar`     | 3215                | 1.0x                    |
| `scalar_par` | 186                 | 17.0x                   |
| `vector`     | 802                 | 4.0x                    |
| `vector_par` | 106                 | 30.3x                   |
| `tiled`      | 770                 | 4.2x                    |
| `tiled_par`  | 102                 | 32.1x                   |
| `ispc`       | 491                 | 6.5x                    |
| `ispc_tasks` | 153                 | 21.7x                   |

`tiled_par` is 1.5x faster than `ispc_tasks`.

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
