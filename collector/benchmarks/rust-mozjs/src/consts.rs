/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use jsapi::{JSCLASS_GLOBAL_APPLICATION_SLOTS, JSCLASS_RESERVED_SLOTS_SHIFT};
use jsapi::{JSCLASS_RESERVED_SLOTS_WIDTH, JSProtoKey};
use libc::c_uint;

pub const default_heapsize: u32 = 32_u32 * 1024_u32 * 1024_u32;

pub const JSCLASS_IS_DOMJSCLASS: c_uint = 1 << 4;
pub const JSCLASS_USERBIT1: c_uint = 1 << 7;

pub const JSCLASS_RESERVED_SLOTS_MASK: c_uint =
    (1 << JSCLASS_RESERVED_SLOTS_WIDTH) - 1;

pub const JSCLASS_HIGH_FLAGS_SHIFT: c_uint =
    JSCLASS_RESERVED_SLOTS_SHIFT + JSCLASS_RESERVED_SLOTS_WIDTH;

pub const JSCLASS_IS_GLOBAL: c_uint =
    1 << (JSCLASS_HIGH_FLAGS_SHIFT + 1);

pub const JSCLASS_IS_PROXY: c_uint =
    1 << (JSCLASS_HIGH_FLAGS_SHIFT + 4);

pub const JSCLASS_GLOBAL_SLOT_COUNT: c_uint =
    JSCLASS_GLOBAL_APPLICATION_SLOTS + JSProtoKey::JSProto_LIMIT as c_uint * 3 + 36;
