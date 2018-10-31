# Stencil

This is the generic [`stencil` ISPC benchmark][ispc]. 

## Usage

```
cargo run --release --features=ispc
```

will run all benchmarks including the ISPC ones. 


## Results

```
./benchmark.sh
```

On a dual core AVX1 i5 @1.8 GHz:

| 800 x 600    | time [ms] <br> Rust | speedup vs `scalar` [-] |
|--------------|---------------------|-------------------------|
| `scalar`     |                2842 |                    1.0x |
| `vector`     |                 630 |                    4.5x |
| `vector_par` |                 444 |                    6.4x |
| `ispc`       |                 558 |                     5.0x |
| `ispc_tasks` |                 470 |                    6.0x |

`vector_par` is 1.06x faster than `ispc_tasks`.

On a 28 core Xeon CPU E5-2690 v4 @ 2.60GHz:

| 800 x 600    | time [ms] <br> Rust | speedup vs `scalar` [-] |
|--------------|---------------------|-------------------------|
| `scalar`     |                1499 | 1.0x                    |
| `vector`     |                 276 | 5.4x                    |
| `vector_par` |                 167 | 9.0x                    |
| `ispc`       |                 287 | 5.2x                    |
| `ispc_tasks` |                 395 | 3.8x                    |

`vector_par` is 1.72x faster than `ispc_tasks`.

On a 40 core Xeon Gold 6148 CPU @ 2.40GHz:

| 800 x 600    | time [ms] <br> Rust | speedup vs `scalar` [-] |
|--------------|---------------------|-------------------------|
| `scalar`     |                1654 |                    1.0x |
| `vector`     |                 278 |                    6.0x |
| `vector_par` |                 148 |                    11.2x |
| `ispc`       |                 185 |                     9.0x |
| `ispc_tasks` |                 401 |                    4.1x |

`vector_par` is 1.25x faster than `ispc`.


[ispc]: https://github.com/ispc/ispc/tree/master/examples/stencil
