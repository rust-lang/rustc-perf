// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(feature = "unstable", feature(nonzero))]
#![cfg_attr(all(test, feature = "unstable"), feature(test))]
#![cfg_attr(test, deny(warnings))]

#[cfg(feature = "unstable")] extern crate core;
#[macro_use] extern crate mac;
extern crate futf;
extern crate utf8;

#[cfg(all(test, feature = "unstable"))]
extern crate test;

pub use tendril::{Tendril, ByteTendril, StrTendril, SliceExt, ReadExt, SubtendrilError};
pub use tendril::{SendTendril, Atomicity, Atomic, NonAtomic};
pub use fmt::Format;
pub use stream::TendrilSink;

pub mod fmt;
pub mod stream;

/// Re-export the rust-encoding crate.
pub mod encoding {
    extern crate encoding;
    pub use self::encoding::*;
}

mod util;
mod buf32;
mod tendril;

static OFLOW: &'static str = "tendril: overflow in buffer arithmetic";
