# Options Pricing ISPC example

This is the [`options` ISPC benchmark][ispc]:

> This program implements both the Black-Scholes and 
> Binomial options pricing models.

## Usage

```
cargo run --release --features=ispc -- ${SIZE} ${ALGORITHM}
```

## Results

```
./benchmark.sh
```

## Black-Scholes

On a dual core AVX1 i5 @1.8 GHz:

| 800 x 800    | time [ms] <br> Rust | speedup vs `scalar` [-] |
|--------------|---------------------|-------------------------|
| `scalar`     |                998 | 1.0x                       |
| `simd`       |                367 | 2.7x                       |
| `par_simd`   |               246 | 4.1x                       |
| `ispc`       |                360 | 2.8x                       |
| `ispc+tasks` |               248 | 4.0x                       |

`par_simd` and `ispc+tasks` algorithms are on par.

## Binomial put

On a dual core AVX1 i5 @1.8 GHz:

| 800 x 800    | time [ms] <br> Rust | speedup vs `scalar` [-] |
|--------------|---------------------|-------------------------|
| `scalar`     |               2057 | 1.0x                       |
| `simd`       |               651 | 3.2x                       |
| `par_simd`   |               279 | 4.3x                       |
| `ispc`       |                805 | 7.4x                       |
| `ispc+tasks` |               404 | 5.1x                       |

`par_simd` algorithm is ~1.4x faster than `ispc+tasks`.


[ispc]: https://github.com/ispc/ispc/tree/master/examples/options
