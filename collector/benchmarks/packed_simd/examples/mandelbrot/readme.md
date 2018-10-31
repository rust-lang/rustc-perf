# Mandelbrot

This is the [`mandelbrot` benchmark from the benchmarksgame][bg].

## Background

http://mathworld.wolfram.com/MandelbrotSet.html

## Usage

It takes four arguments in this order:

* `width`: width of the image to render
* `height`: height of the image to render
* `algorithm`: algorithm to use:
  * `scalar`: scalar algorithm
  * `simd`: parallelized SIMD algorithm
  * `ispc`: ISPC + tasks algorithm
* `--color` (optional): enables colorized output, which also determines the image format.
  * disabled (default): PBM: Portable BitMap format (black & white output)
  * enabled: PPM: Portable PixMap format (colored output)

The resulting image is piped to `stdout`.

`cargo run --release -- 400 400 --algo simd > output.ppm` outputs:

![run_400_png](https://user-images.githubusercontent.com/904614/43190942-72bdb834-8ffa-11e8-9dcf-a9a9632ae907.png)

`cargo run --release -- 400 400 --algo simd --color > output.ppm` outputs:

![run_400_400_1_1_png](https://user-images.githubusercontent.com/904614/43190948-759969a4-8ffa-11e8-81a9-35e5baef3e86.png)

## Performance

```
./benchmark.sh
```

On a dual core AVX1 i5 @1.8 GHz:

| 800 x 800  | time [ms] <br> Rust | speedup vs `scalar` [-] |
|------------|---------------------|-------------|
| `scalar`   | 86.6                | 1.0x        |
| `simd`     | 21.0                | 4.1x        |
| `ispc`     | 25.7                | 3.4x        |

`simd` algorithm is ~1.2x faster than `ispc`.

On a 28 core Xeon CPU E5-2690 v4 @ 2.60GHz:

| 800 x 800  | time [ms] <br> Rust | speedup vs `scalar` [-] |
|------------|---------------------|-------------------------|
| `scalar`   | 50.8                | 1.0x                    |
| `simd`     | 25.1                | 2x                      |
| `ispc`     | 14.4                | 3.52x                   |

`simd` algorithm is ~1.74x slower than `ispc`.

On a 40 core Xeon Gold 6148 CPU @ 2.40GHz:

| 800 x 800  | time [ms] <br> Rust | speedup vs `scalar` [-] |
|------------|---------------------|-------------|
| `scalar`   | 59.9                | 1.0x        |
| `simd`     | 29.9                | 2.0x        |
| `ispc`     | 30.3                | 2.0x        |

`simd` algorithm is as fast as `ispc`.

[bg]: https://benchmarksgame-team.pages.debian.net/benchmarksgame/description/mandelbrot.html#mandelbrot
