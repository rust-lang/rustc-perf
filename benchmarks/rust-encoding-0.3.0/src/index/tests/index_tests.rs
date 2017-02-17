// This is a part of rust-encoding.
// Copyright (c) 2013-2015, Kang Seonghoon.
// See README.md and LICENSE.txt for details.

//! Macros and utilities for testing indices.

/// Makes a common test suite for single-byte indices.
#[macro_export]
macro_rules! single_byte_tests {
    () => (
        mod tests {
            extern crate test;
            use super::{forward, backward};

            #[test]
            fn test_correct_table() {
                for i in 0x80..0x100 {
                    let i = i as u8;
                    let j = forward(i);
                    if j != 0xffff { assert_eq!(backward(j as u32), i); }
                }
                for i in 0..0x110000 {
                    let j = backward(i);
                    if j != 0 { assert_eq!(forward(j) as u32, i); }
                }
            }

            #[bench]
            fn bench_forward_sequential_128(bencher: &mut test::Bencher) {
                bencher.iter(|| {
                    for i in 0x80..0x100 {
                        test::black_box(forward(i as u8));
                    }
                })
            }

            // strongly prefers the performance for invalid characters
            #[bench]
            fn bench_backward_sequential_128(bencher: &mut test::Bencher) {
                let mut start: u32 = 0;
                bencher.iter(|| {
                    for i in start..(start + 0x80) {
                        test::black_box(backward(i));
                    }
                    start += 0x80;
                })
            }

            // strongly prefers the performance for valid characters
            #[bench]
            fn bench_roundtrip_sequential_128(bencher: &mut test::Bencher) {
                bencher.iter(|| {
                    for i in 0x80..0x100 {
                        let j = forward(i as u8);
                        if j != 0 { test::black_box(backward(j as u32)); }
                    }
                })
            }
        }
    );
}

/// Makes a common test suite for multi-byte indices.
#[macro_export]
macro_rules! multi_byte_tests {
    (make shared tests and benches with dups = $dups:expr) => ( // internal macro
        #[test]
        fn test_correct_table() {
            const DUPS: &'static [u16] = &$dups;
            for i in 0..0x10000 {
                let i = i as u16;
                if DUPS.contains(&i) { continue; }
                let j = forward(i);
                if j != 0xffff { assert_eq!(backward(j), i); }
            }
            for i in 0..0x110000 {
                let j = backward(i);
                if DUPS.contains(&j) { continue; }
                if j != 0xffff { assert_eq!(forward(j), i); }
            }
        }

        #[bench]
        fn bench_forward_sequential_128(bencher: &mut test::Bencher) {
            let mut start: u32 = 0;
            bencher.iter(|| {
                for i in start..(start + 0x80) {
                    test::black_box(forward(i as u16));
                }
                start += 0x80;
            })
        }

        #[bench]
        fn bench_backward_sequential_128(bencher: &mut test::Bencher) {
            let mut start: u32 = 0;
            bencher.iter(|| {
                for i in start..(start + 0x80) {
                    test::black_box(backward(i));
                }
                start += 0x80;
                if start >= 0x110000 { start = 0; }
            })
        }
    );

    (
        dups = $dups:expr
    ) => (
        mod tests {
            extern crate test;
            use super::{forward, backward};

            multi_byte_tests!(make shared tests and benches with dups = $dups);
        }
    );

    (
        remap = [$remap_min:expr, $remap_max:expr],
        dups = $dups:expr
    ) => (
        mod tests {
            extern crate test;
            use super::{forward, backward, backward_remapped};

            multi_byte_tests!(make shared tests and benches with dups = $dups);

            static REMAP_MIN: u16 = $remap_min;
            static REMAP_MAX: u16 = $remap_max;

            #[test]
            fn test_correct_remapping() {
                for i in REMAP_MIN..(REMAP_MAX+1) {
                    let j = forward(i);
                    if j != 0xffff {
                        let ii = backward_remapped(j);
                        assert!(ii != i && ii != 0xffff);
                        let jj = forward(ii);
                        assert_eq!(j, jj);
                    }
                }
            }

            #[bench]
            fn bench_backward_remapped_sequential_128(bencher: &mut test::Bencher) {
                let mut start: u32 = 0;
                bencher.iter(|| {
                    for i in start..(start + 0x80) {
                        test::black_box(backward_remapped(i));
                    }
                    start += 0x80;
                    if start >= 0x110000 { start = 0; }
                })
            }
        }
    );
}

/// Makes a common test suite for multi-byte range indices.
#[macro_export]
macro_rules! multi_byte_range_tests {
    (
        key = [$minkey:expr, $maxkey:expr], key < $keyubound:expr,
        value = [$minvalue:expr, $maxvalue:expr], value < $valueubound:expr
    ) => (
        mod tests {
            extern crate test;
            use super::{forward, backward};

            static MIN_KEY: u32 = $minkey;
            static MAX_KEY: u32 = $maxkey;
            static KEY_UBOUND: u32 = $keyubound;
            static MIN_VALUE: u32 = $minvalue;
            static MAX_VALUE: u32 = $maxvalue;
            static VALUE_UBOUND: u32 = $valueubound;

            #[test]
            #[allow(unused_comparisons)]
            fn test_no_failure() {
                for i in MIN_KEY.saturating_sub(1)..(MAX_KEY+2) {
                    forward(i);
                }
                for j in MIN_VALUE.saturating_sub(1)..(MAX_VALUE+2) {
                    backward(j);
                }
            }

            #[test]
            fn test_correct_table() {
                for i in MIN_KEY..(MAX_KEY+2) {
                    let j = forward(i);
                    if j == 0xffffffff { continue; }
                    let i_ = backward(j);
                    if i_ == 0xffffffff { continue; }
                    assert!(i_ == i,
                            "backward(forward({})) = backward({}) = {} != {}", i, j, i_, i);
                }
            }

            #[bench]
            fn bench_forward_sequential_128(bencher: &mut test::Bencher) {
                let mut start: u32 = 0;
                bencher.iter(|| {
                    for i in start..(start + 0x80) {
                        test::black_box(forward(i));
                    }
                    start += 0x80;
                    if start >= KEY_UBOUND { start = 0; }
                })
            }

            #[bench]
            fn bench_backward_sequential_128(bencher: &mut test::Bencher) {
                let mut start: u32 = 0;
                bencher.iter(|| {
                    for i in start..(start + 0x80) {
                        test::black_box(backward(i));
                    }
                    start += 0x80;
                    if start >= VALUE_UBOUND { start = 0; }
                })
            }
        }
    );
}

