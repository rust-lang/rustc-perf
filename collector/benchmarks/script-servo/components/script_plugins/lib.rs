/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Servo's compiler plugin/macro crate
//!
//! Attributes this crate provides:
//!
//!  - `#[derive(DenyPublicFields)]` : Forces all fields in a struct/enum to be private
//!  - `#[derive(JSTraceable)]` : Auto-derives an implementation of `JSTraceable` for a struct in the script crate
//!  - `#[must_root]` : Prevents data of the marked type from being used on the stack.
//!                     See the lints module for more details
//!  - `#[dom_struct]` : Implies #[derive(JSTraceable, DenyPublicFields)]`, and `#[must_root]`.
//!                       Use this for structs that correspond to a DOM type



#![deny(unsafe_code)]
#![feature(plugin)]
#![feature(plugin_registrar)]
#![feature(rustc_private)]

#[cfg(feature = "unrooted_must_root_lint")]
#[macro_use]
extern crate rustc;

extern crate rustc_plugin;
extern crate syntax;
extern crate syntax_pos;

use rustc_plugin::Registry;
use syntax::feature_gate::AttributeType::Whitelisted;
use syntax_pos::symbol::Symbol;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_attribute(Symbol::intern("allow_unrooted_interior"), Whitelisted);
    reg.register_attribute(Symbol::intern("must_root"), Whitelisted);
}
