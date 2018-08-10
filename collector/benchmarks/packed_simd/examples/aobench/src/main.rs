//! aobench: Ambient Occlusion Renderer benchmark.
//!
//! Based on: https://code.google.com/archive/p/aobench/

#[macro_use]
extern crate structopt;
extern crate aobench_lib;
extern crate time;

use aobench_lib::*;
use std::path::PathBuf;
use structopt::StructOpt;

/// Command-line arguments.
#[derive(StructOpt, Debug)]
struct Opt {
    /// Image width.
    width: usize,
    /// Image height.
    height: usize,

    /// Algorithm
    #[structopt(short = "a", long = "algo")]
    algo: String,

    /// Output file.
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    let mut scene = aobench_lib::scene::Random::new();
    let mut img = Image::new(opt.width, opt.height);

    let d = match opt.algo.as_str() {
        "scalar" => {
            let d =
                time::Duration::span(|| scalar::ao(&mut scene, 2, &mut img));
            let image_path =
                opt.output.unwrap_or(PathBuf::from("image_scalar.png"));
            img.write_png(&image_path, false)
                .expect("failed to write image");
            d
        }
        "scalar_par" => {
            let d = time::Duration::span(|| {
                scalar_parallel::ao(&mut scene, 2, &mut img)
            });
            let image_path =
                opt.output.unwrap_or(PathBuf::from("image_scalar_par.png"));
            img.write_png(&image_path, false)
                .expect("failed to write image");
            d
        }
        "vector" => {
            let d =
                time::Duration::span(|| vector::ao(&mut scene, 2, &mut img));
            let image_path =
                opt.output.unwrap_or(PathBuf::from("image_vector.png"));
            img.write_png(&image_path, false)
                .expect("failed to write image");
            d
        }
        "vector_par" => {
            let d = time::Duration::span(|| {
                vector_parallel::ao(&mut scene, 2, &mut img)
            });
            let image_path =
                opt.output.unwrap_or(PathBuf::from("image_vector_par.png"));
            img.write_png(&image_path, false)
                .expect("failed to write image");
            d
        }
        v => {
            panic!("unknown algorithm: {}", v);
        }
    };
    println!("time: {} ms", d.num_milliseconds());
}
