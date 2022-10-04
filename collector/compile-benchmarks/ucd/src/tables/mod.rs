pub mod general;
pub use self::general::*;

pub mod bidi;
pub use self::bidi::*;

pub mod misc;
pub use self::misc::*;

pub mod scripts;
pub use self::scripts::*;

pub mod function;
pub use self::function::*;

pub mod bool;
pub use self::bool::*;

pub mod rem;
pub use self::rem::*;

pub mod decomp;
pub use self::decomp::*;

use core::cmp::Ordering::{Equal, Less, Greater};
use core::char;

pub trait Search {
    type T: Clone;
    fn search(&self, cp: char) -> Option<Self::T>;
    fn includes(&self, cp: char) -> bool {
        match self.search(cp) {
            Some(_) => true,
            None => false
        }
    }
}

impl<S: Clone> Search for [((u8,u8,u8),(u8,u8,u8),S)] {
    type T = S;
    fn search(&self, cp: char) -> Option<S> {
        let cp = cp as u32;
        match self.binary_search_by(|&((rb1,rb2,rb3), (re1,re2,re3), _)| {
            let rb: u32 = (rb1 as u32)*65536 + (rb2 as u32)*256 + (rb3 as u32);
            let re: u32 = (re1 as u32)*65536 + (re2 as u32)*256 + (re3 as u32);
            if rb <= cp && cp <= re { Equal }
            else if re < cp { Less }
            else { Greater }
        }) {
            Ok(idx) => {
                let (_, _, ref v) = self[idx];
                Some(v.clone())
            },
            _ => None
        }
    }
}

impl<S: Clone> Search for [((u8,u8,u8),S)] {
    type T = S;
    fn search(&self, cp: char) -> Option<S> {
        let ca = cp as u32;
        match self.binary_search_by(|&((cb1,cb2,cb3), _)| {
            let cb: u32 = (cb1 as u32)*65536 + (cb2 as u32)*256 + (cb3 as u32);
            cb.cmp(&ca)
        }) {
            Ok(idx) => {
                let (_, ref v) = self[idx];
                Some(v.clone())
            },
            _ => None
        }
    }
}

impl Search for [(u16,u16)] {
    type T = char;
    fn search(&self, cp: char) -> Option<char> {
        let ca = cp as u32;
        let cb = ca as u16;

        if ca > 65536 { return None; }

        match self.binary_search_by(|&(cc,_)| cc.cmp(&cb)) {
            Ok(idx) => {
                let (_, v) = self[idx];
                char::from_u32(v as u32)
            },
            _ => None
        }
    }
}