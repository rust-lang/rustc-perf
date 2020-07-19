/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/. */


use jsapi::{JSObject, JSString};
use jsapi::Value;
use jsapi::jsval_layout;
use jsapi::JSValueType;

use libc::c_void;
use std::default::Default;
use std::mem;
use std::ptr;

pub type JSVal = Value;

#[cfg(target_pointer_width = "64")]
const JSVAL_TAG_SHIFT: usize = 47;

#[cfg(target_pointer_width = "64")]
const JSVAL_TAG_MAX_DOUBLE: u32 = 0x1FFF0u32;

#[cfg(target_pointer_width = "32")]
const JSVAL_TAG_CLEAR: u32 = 0xFFFFFF80;

#[cfg(target_pointer_width = "64")]
#[repr(u32)]
#[allow(dead_code)]
enum ValueTag {
    INT32     = JSVAL_TAG_MAX_DOUBLE | (JSValueType::JSVAL_TYPE_INT32 as u32),
    UNDEFINED = JSVAL_TAG_MAX_DOUBLE | (JSValueType::JSVAL_TYPE_UNDEFINED as u32),
    STRING    = JSVAL_TAG_MAX_DOUBLE | (JSValueType::JSVAL_TYPE_STRING as u32),
    SYMBOL    = JSVAL_TAG_MAX_DOUBLE | (JSValueType::JSVAL_TYPE_SYMBOL as u32),
    BOOLEAN   = JSVAL_TAG_MAX_DOUBLE | (JSValueType::JSVAL_TYPE_BOOLEAN as u32),
    MAGIC     = JSVAL_TAG_MAX_DOUBLE | (JSValueType::JSVAL_TYPE_MAGIC as u32),
    NULL      = JSVAL_TAG_MAX_DOUBLE | (JSValueType::JSVAL_TYPE_NULL as u32),
    OBJECT    = JSVAL_TAG_MAX_DOUBLE | (JSValueType::JSVAL_TYPE_OBJECT as u32),
}

#[cfg(target_pointer_width = "32")]
#[repr(u32)]
#[allow(dead_code)]
enum ValueTag {
    PRIVATE   = 0,
    INT32     = JSVAL_TAG_CLEAR as u32 | (JSValueType::JSVAL_TYPE_INT32 as u32),
    UNDEFINED = JSVAL_TAG_CLEAR as u32 | (JSValueType::JSVAL_TYPE_UNDEFINED as u32),
    STRING    = JSVAL_TAG_CLEAR as u32 | (JSValueType::JSVAL_TYPE_STRING as u32),
    SYMBOL    = JSVAL_TAG_CLEAR as u32 | (JSValueType::JSVAL_TYPE_SYMBOL as u32),
    BOOLEAN   = JSVAL_TAG_CLEAR as u32 | (JSValueType::JSVAL_TYPE_BOOLEAN as u32),
    MAGIC     = JSVAL_TAG_CLEAR as u32 | (JSValueType::JSVAL_TYPE_MAGIC as u32),
    NULL      = JSVAL_TAG_CLEAR as u32 | (JSValueType::JSVAL_TYPE_NULL as u32),
    OBJECT    = JSVAL_TAG_CLEAR as u32 | (JSValueType::JSVAL_TYPE_OBJECT as u32),
}

#[cfg(target_pointer_width = "64")]
#[repr(u64)]
#[allow(dead_code)]
enum ValueShiftedTag {
    MAX_DOUBLE = (((JSVAL_TAG_MAX_DOUBLE as u64) << JSVAL_TAG_SHIFT) | 0xFFFFFFFFu64),
    INT32      = ((ValueTag::INT32 as u64)      << JSVAL_TAG_SHIFT),
    UNDEFINED  = ((ValueTag::UNDEFINED as u64)  << JSVAL_TAG_SHIFT),
    STRING     = ((ValueTag::STRING as u64)     << JSVAL_TAG_SHIFT),
    SYMBOL     = ((ValueTag::SYMBOL as u64)     << JSVAL_TAG_SHIFT),
    BOOLEAN    = ((ValueTag::BOOLEAN as u64)    << JSVAL_TAG_SHIFT),
    MAGIC      = ((ValueTag::MAGIC as u64)      << JSVAL_TAG_SHIFT),
    NULL       = ((ValueTag::NULL as u64)       << JSVAL_TAG_SHIFT),
    OBJECT     = ((ValueTag::OBJECT as u64)     << JSVAL_TAG_SHIFT),
}


#[cfg(target_pointer_width = "64")]
const JSVAL_PAYLOAD_MASK: u64 = 0x00007FFFFFFFFFFF;

#[inline(always)]
fn AsJSVal(val: u64) -> JSVal {
    JSVal {
        data: jsval_layout {
            asBits: val,
        }
    }
}

#[cfg(target_pointer_width = "64")]
#[inline(always)]
fn BuildJSVal(tag: ValueTag, payload: u64) -> JSVal {
    AsJSVal(((tag as u32 as u64) << JSVAL_TAG_SHIFT) | payload)
}

#[cfg(target_pointer_width = "32")]
#[inline(always)]
fn BuildJSVal(tag: ValueTag, payload: u64) -> JSVal {
    AsJSVal(((tag as u32 as u64) << 32) | payload)
}

#[inline(always)]
pub fn NullValue() -> JSVal {
    BuildJSVal(ValueTag::NULL, 0)
}

#[inline(always)]
pub fn UndefinedValue() -> JSVal {
    BuildJSVal(ValueTag::UNDEFINED, 0)
}

#[inline(always)]
pub fn Int32Value(i: i32) -> JSVal {
    BuildJSVal(ValueTag::INT32, i as u32 as u64)
}

#[cfg(target_pointer_width = "64")]
#[inline(always)]
pub fn DoubleValue(f: f64) -> JSVal {
    let bits: u64 = unsafe { mem::transmute(f) };
    assert!(bits <= ValueShiftedTag::MAX_DOUBLE as u64);
    AsJSVal(bits)
}

#[cfg(target_pointer_width = "32")]
#[inline(always)]
pub fn DoubleValue(f: f64) -> JSVal {
    let bits: u64 = unsafe { mem::transmute(f) };
    let val = AsJSVal(bits);
    assert!(val.is_double());
    val
}

#[inline(always)]
pub fn UInt32Value(ui: u32) -> JSVal {
    if ui > 0x7fffffff {
        DoubleValue(ui as f64)
    } else {
        Int32Value(ui as i32)
    }
}

#[cfg(target_pointer_width = "64")]
#[inline(always)]
pub fn StringValue(s: &JSString) -> JSVal {
    let bits = s as *const JSString as usize as u64;
    assert!((bits >> JSVAL_TAG_SHIFT) == 0);
    BuildJSVal(ValueTag::STRING, bits)
}

#[cfg(target_pointer_width = "32")]
#[inline(always)]
pub fn StringValue(s: &JSString) -> JSVal {
    let bits = s as *const JSString as usize as u64;
    BuildJSVal(ValueTag::STRING, bits)
}

#[inline(always)]
pub fn BooleanValue(b: bool) -> JSVal {
    BuildJSVal(ValueTag::BOOLEAN, b as u64)
}

#[cfg(target_pointer_width = "64")]
#[inline(always)]
pub fn ObjectValue(o: *mut JSObject) -> JSVal {
    let bits = o as usize as u64;
    assert!((bits >> JSVAL_TAG_SHIFT) == 0);
    BuildJSVal(ValueTag::OBJECT, bits)
}

#[cfg(target_pointer_width = "32")]
#[inline(always)]
pub fn ObjectValue(o: *mut JSObject) -> JSVal {
    let bits = o as usize as u64;
    BuildJSVal(ValueTag::OBJECT, bits)
}

#[inline(always)]
pub fn ObjectOrNullValue(o: *mut JSObject) -> JSVal {
    if o.is_null() {
        NullValue()
    } else {
        ObjectValue(o)
    }
}

#[cfg(target_pointer_width = "64")]
#[inline(always)]
pub fn PrivateValue(o: *const c_void) -> JSVal {
    let ptrBits = o as usize as u64;
    assert!((ptrBits & 1) == 0);
    AsJSVal(ptrBits >> 1)
}

#[cfg(target_pointer_width = "32")]
#[inline(always)]
pub fn PrivateValue(o: *const c_void) -> JSVal {
    let ptrBits = o as usize as u64;
    assert!((ptrBits & 1) == 0);
    BuildJSVal(ValueTag::PRIVATE, ptrBits)
}
