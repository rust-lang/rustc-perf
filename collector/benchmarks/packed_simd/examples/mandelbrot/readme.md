# Mandelbrot

This is the [`mandelbrot` benchmark from the benchmarksgame][bg]. 

## Background

http://mathworld.wolfram.com/MandelbrotSet.html

## Usage

It takes four arguments in this order:

* `width`: width of the image to render
* (optional) `height`: height of the image to render - defaults to `width`
* (optional) `algorithm`: algorithm to use - defaults to the fastest one.
  * `0`: scalar algorithm
  * `1`: SIMD algorithm
* (optional) `output_format`: the output format to use - defaults to `PBM`
  * `0`: PBM: Portable BitMap format (black & white output)
  * `1`: PPM: Portable PixMap format (colored output)
  
`cargo run --release -- 400` outputs:

![run_400_png](https://user-images.githubusercontent.com/904614/43190942-72bdb834-8ffa-11e8-9dcf-a9a9632ae907.png)

`cargo run --releae -- 400 400 1 1` outputs:

![run_400_400_1_1_png](https://user-images.githubusercontent.com/904614/43190948-759969a4-8ffa-11e8-81a9-35e5baef3e86.png)

[bg]: https://benchmarksgame-team.pages.debian.net/benchmarksgame/description/mandelbrot.html#mandelbrot
