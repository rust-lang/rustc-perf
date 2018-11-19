//! The N-body benchmark from the [benchmarks game][bg].
//!
//! [bg]: https://benchmarksgame-team.pages.debian.net/benchmarksgame/description/nbody.html#nbody
#![deny(warnings, rust_2018_idioms)]

fn run<O: std::io::Write>(o: &mut O, n: usize, alg: usize) {
    let (energy_before, energy_after) = nbody_lib::run(n, alg);

    writeln!(o, "{:.9}", energy_before);
    writeln!(o, "{:.9}", energy_after);
}

fn main() {
    let n: usize = std::env::args()
        .nth(1)
        .expect("need one arg")
        .parse()
        .expect("argument should be a usize");

    let alg: usize = if let Some(v) = std::env::args().nth(2) {
        v.parse().expect("second argument must be a usize")
    } else {
        1 // SIMD algorithm
    };

    run(&mut std::io::stdout(), n, alg);
}

#[cfg(test)]
mod tests {
    use super::*;
    static OUTPUT: &'static [u8] = include_bytes!("nbody-output.txt");
    #[test]
    fn verify_output_simd() {
        let mut out: Vec<u8> = Vec::new();

        run(&mut out, 1000, 0);

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
    fn verify_output_scalar() {
        let mut out: Vec<u8> = Vec::new();

        run(&mut out, 1000, 1);

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
