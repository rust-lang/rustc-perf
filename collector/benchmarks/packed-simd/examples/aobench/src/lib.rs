//! aobench: Ambient Occlusion Renderer benchmark.
//!
//! Based on [aobench](https://code.google.com/archive/p/aobench/) by Syoyo
//! Fujita.
#![deny(warnings, rust_2018_idioms)]
#![allow(non_snake_case, non_camel_case_types)]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(
        clippy::many_single_char_names,
        clippy::similar_names,
        clippy::cast_precision_loss,
        clippy::inline_always,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::identity_op,
        clippy::erasing_op
    )
)]

pub mod ambient_occlusion;
pub mod geometry;
pub mod image;
pub mod intersection;
pub mod random;
pub mod scene;

#[cfg(feature = "ispc")]
pub mod ispc_;
pub mod scalar;
pub mod scalar_parallel;
pub mod tiled;
pub mod tiled_parallel;
pub mod vector;
pub mod vector_parallel;

pub use self::image::Image;
pub use self::scene::Scene;
