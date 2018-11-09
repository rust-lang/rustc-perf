#![deny(warnings, rust_2018_idioms)]
#![cfg_attr(test, feature(avx512_target_feature, abi_vectorcall))]

#[cfg(test)]
mod api;
