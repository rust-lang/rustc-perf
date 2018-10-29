//! The Mandelbrot benchmark from the [benchmarksgame][bg]
//!
//! [bg]: https://benchmarksgame-team.pages.debian.net/benchmarksgame/description/mandelbrot.html#mandelbrot

#![deny(warnings, rust_2018_idioms)]

use mandelbrot_lib::*;
use std::io;
use structopt::StructOpt;

/// Mandelbrot image generator.
///
/// Output is printed to `stdout`.
#[derive(StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
struct Opt {
    /// Image width.
    width: usize,
    /// Image height.
    height: usize,

    /// Enable this to output a color image.
    #[structopt(short = "c", long = "color")]
    color: bool,

    /// Algorithm
    #[structopt(short = "a", long = "algo")]
    algo: String,
}

const ALGORITHMS: &[&str] = &["scalar", "simd", "ispc"];

fn main() {
    let opt = Opt::from_args();

    let algo = match opt.algo.as_str() {
        "scalar" => Algorithm::Scalar,
        "simd" => Algorithm::Simd,
        "ispc" => Algorithm::Ispc,
        algo => panic!(
            "Unknown algorithm: {:?}\nAvailable algorithms: {:?}",
            algo, ALGORITHMS
        ),
    };

    let mb = Mandelbrot::generate((opt.width, opt.height), algo);

    let mut stdout = io::stdout();
    if opt.color {
        mb.output_ppm(&mut stdout).unwrap();
    } else {
        mb.output_pbm(&mut stdout).unwrap();
    }
}
