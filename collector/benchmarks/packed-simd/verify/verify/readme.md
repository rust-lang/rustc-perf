# Machine code verification

## Quick start

To run the verification tests run:

```
cargo test --release
```

on this crate, eventually passing the required target features via `RUSTFLAGS`.
For example, `RUSTFLAGS="-C target-feature=+avx2"`.

This crate only contains tests, and the tests only run in `--release` mode.
Therefore, building this crate with anything different from `cargo test
--release` does not make much sense.

## How it works

This crates verifies the machine code generated for some of the portable packed
vector APIs by disassembling the API at run-time and comparing the machine code
generated against the desired one for a particular target and target features.

This is done by using the
[`stdsimd-test`](https://github.com/rust-lang-nursery/stdsimd/tree/master/crates/stdsimd-test)
crate, which exposes the `assert_instr` procedural macro. It is used like this:

```rust
// The verification functions must be #[inline]: 
#[inline]
// Enable the target features required for the desired code generation
// on the different targets:
#[cfg_attr(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature(enable = "avx512f,avx512vl")
)]
// Check that the disassembly contains a particular instruction:
#[cfg_attr(
    any(target_arch = "x86", target_arch = "x86_64"),
    assert_instr(vpro)
)]
unsafe fn rotate_right_variable(x: u64x8, v: u64x8) -> u64x8 {
    x.rotate_right(v)
}
```

The `assert_instr` procedural macro creates a test that contains a
`#[inline(never)]` function that calls the API. It then gets a function pointer
to this function, and calls `stdsimd_test::assert` with it, the function name,
and the expected assembly instruction. `stdsimd_test` uses `objdump` or similar
to disassemble itself, it then looks for the function address and name in the
disassembly, and verifies that the machine code for the function contains the
instruction.
