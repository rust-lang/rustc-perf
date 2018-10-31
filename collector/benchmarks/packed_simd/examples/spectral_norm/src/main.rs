#![deny(warnings)]

extern crate spectral_norm_lib;
use spectral_norm_lib::*;

fn run<O: std::io::Write>(o: &mut O, n: usize, alg: usize) {
    let answer = spectral_norm(n, alg);
    writeln!(o, "{:.9}", answer).unwrap();
}

fn main() {
    let n: usize =
        std::env::args().nth(1).expect("need one arg").parse().unwrap();

    let alg = if let Some(v) = std::env::args().nth(2) {
        v.parse().unwrap()
    } else {
        0
    };

    run(&mut std::io::stdout(), n, alg);
}

#[cfg(test)]
mod tests {
    use super::*;
    static OUTPUT: &'static [u8] = include_bytes!("spectralnorm-output.txt");
    #[test]
    fn verify_output_simd() {
        let mut out: Vec<u8> = Vec::new();

        run(&mut out, 100, 0);

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

        run(&mut out, 100, 1);

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
