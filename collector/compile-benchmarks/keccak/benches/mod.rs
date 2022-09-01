#![feature(test)]
extern crate keccak;
extern crate test;

#[bench]
fn f1600(b: &mut test::Bencher) {
    let mut data = [0u64; 25];
    b.iter(|| keccak::f1600(&mut data));
}
