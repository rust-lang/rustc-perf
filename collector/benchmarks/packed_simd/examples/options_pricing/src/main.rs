#![deny(warnings, rust_2018_idioms)]
#![feature(custom_inner_attributes)]

use options_pricing_lib::*;

#[rustfmt::skip]
fn run<F>(name: &str, count: usize, f: F)
where
    F: Fn(&[f32], &[f32], &[f32], &[f32], &[f32], &mut [f32], usize) -> f64,
{
    let mut d = State::new(count);
    let t = time::Duration::span(move || { d.exec(f); } );
    println!("{}: {} ms", name, t.num_milliseconds());
}

macro_rules! ispc_alg {
    ($name:tt, $count:ident, $fun:path) => {{
        #[cfg(feature = "ispc")]
        {
            run($name, $count, $fun);
        }
        #[cfg(not(feature = "ispc"))]
        {
            panic!("algorithm {} requires --feature=ispc", $name);
        }
    }};
}

fn main() {
    let mut args = std::env::args();
    args.next();
    let num_options: usize = args
        .next()
        .unwrap()
        .parse()
        .expect("expected argument 1 of type usize: num_options");
    let algorithm: String = args
        .next()
        .unwrap()
        .parse()
        .expect("expected argument 2 of type String: algorithm");

    match algorithm.as_str() {
        "black_scholes_ispc_tasks" => ispc_alg!(
            "black_scholes_ispc_tasks",
            num_options,
            ispc_::black_scholes::tasks
        ),
        "black_scholes_ispc" => ispc_alg!(
            "black_scholes_ispc",
            num_options,
            ispc_::black_scholes::serial
        ),
        "binomial_put_ispc_tasks" => ispc_alg!(
            "binomial_put_ispc_tasks",
            num_options,
            ispc_::binomial_put::tasks
        ),
        "binomial_put_ispc" => ispc_alg!(
            "binomial_put_ispc",
            num_options,
            ispc_::binomial_put::serial
        ),
        "black_scholes_scalar" => {
            run("black_scholes_scalar", num_options, scalar::black_scholes)
        }
        "binomial_put_scalar" => {
            run("binomial_put_scalar", num_options, scalar::binomial_put)
        }
        "black_scholes_simd" => {
            run("black_scholes_simd", num_options, simd::black_scholes)
        }
        "binomial_put_simd" => {
            run("binomial_put_simd", num_options, simd::binomial_put)
        }
        "black_scholes_simd_par" => {
            run("black_scholes_simd_par", num_options, simd_par::black_scholes)
        }
        "binomial_put_simd_par" => {
            run("binomial_put_simd_par", num_options, simd_par::binomial_put)
        }
        _ => panic!("unknown algorithm: {}", algorithm),
    }
}
