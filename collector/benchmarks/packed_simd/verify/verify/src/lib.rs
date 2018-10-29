#![deny(warnings, rust_2018_idioms)]
#![cfg_attr(test, feature(plugin, avx512_target_feature, abi_vectorcall))]
#![cfg_attr(test, plugin(interpolate_idents))]

#[cfg(test)]
mod api;
