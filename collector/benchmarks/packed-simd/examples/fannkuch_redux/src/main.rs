#![deny(warnings, rust_2018_idioms)]

use fannkuch_redux_lib::*;

fn run<O: std::io::Write>(o: &mut O, n: usize, alg: usize) {
    let (checksum, maxflips) = fannkuch_redux(n, alg);
    writeln!(o, "{}\nPfannkuchen({}) = {}", checksum, n, maxflips).unwrap();
}

fn main() {
    let n: usize =
        std::env::args().nth(1).expect("need one arg").parse().unwrap();
    assert!(3 <= n && n <= 14, "n = {} is out-of-range [3, 14]", n);
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
    static OUTPUT: &'static [u8] = include_bytes!("fannkuchredux-output.txt");
    #[test]
    fn verify_output_simd() {
        let mut out: Vec<u8> = Vec::new();

        run(&mut out, 7, 0);

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

        run(&mut out, 7, 1);

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
