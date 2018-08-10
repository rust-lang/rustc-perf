//! The mandelbrot benchmark from the [benchmarks game][bg].
//!
//! [bg]: https://benchmarksgame-team.pages.debian.net/benchmarksgame/description/mandelbrot.html#mandelbrot
extern crate mandelbrot_lib;
use mandelbrot_lib::*;
use std::{env, io, io::Write};

enum Algorithm {
    Scalar,
    Simd,
}
use self::Algorithm::*;

fn run<O: Write>(
    mut o: O,
    width: usize,
    height: usize,
    alg: Algorithm,
    format: Format,
) {
    let mut m = Mandelbrot::new(width, height, format);
    m.write_header(&mut o);

    match alg {
        Scalar => scalar::output(&mut o, &mut m, LIMIT),
        Simd => simd::output(&mut o, &mut m, LIMIT),
    }
}

fn main() {
    let mut args = env::args();
    args.next();

    // width height alg fmt
    let width = args.next().unwrap().parse().unwrap();

    let height = if let Some(h) = args.next() {
        h.parse().unwrap()
    } else {
        width
    };

    let alg = if let Some(v) = args.next() {
        match v.parse().unwrap() {
            0 => Scalar,
            1 => Simd,
            v => panic!("unknown algorithm value: {}", v),
        }
    } else {
        Simd
    };

    let fmt = if let Some(f) = args.next() {
        match f.parse().unwrap() {
            0 => output::Format::PBM,
            1 => output::Format::PPM,
            v => panic!("unknown output format value: {}", v),
        }
    } else {
        output::Format::PBM
    };

    run(io::stdout(), width, height, alg, fmt);
}

#[cfg(test)]
mod tests {
    use super::*;
    static OUTPUT: &'static [u8] = include_bytes!("mandelbrot-output.txt");
    const WIDTH: usize = 200;
    const HEIGHT: usize = 200;
    #[test]
    fn verify_output_scalar() {
        let mut out: Vec<u8> = Vec::new();

        run(&mut out, WIDTH, HEIGHT, Scalar, output::Format::PBM);

        assert_eq!(out.len(), OUTPUT.len());
        if out != OUTPUT {
            for i in 0..out.len() {
                assert_eq!(
                    out[i], OUTPUT[i],
                    "byte {} differs - is: {:#08b} - should: {:#08b}",
                    i, out[i], OUTPUT[i]
                );
            }
        }
    }
    #[test]
    fn verify_output_simd() {
        let mut out: Vec<u8> = Vec::new();

        run(&mut out, WIDTH, HEIGHT, Simd, output::Format::PBM);

        assert_eq!(out.len(), OUTPUT.len());
        if out != OUTPUT {
            for i in 0..out.len() {
                assert_eq!(
                    out[i], OUTPUT[i],
                    "byte {} differs - is: {:#08b} - should: {:#08b}",
                    i, out[i], OUTPUT[i]
                );
            }
        }
    }
}
