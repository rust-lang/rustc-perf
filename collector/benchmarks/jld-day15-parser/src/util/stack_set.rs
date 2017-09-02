use std::mem::size_of;
use std::ops::Deref;

pub trait BitSet: Sized {
    type Idx: Copy;
    fn new(n: usize) -> Option<Self>;
    fn is_full(&self) -> bool;
    fn contains(&self, i: Self::Idx) -> bool;
    fn set(&mut self, i: Self::Idx);
    fn clear(&mut self, i: Self::Idx);
}

#[derive(Debug, Clone)]
pub struct StackSet<B: BitSet> {
    stack: Vec<B::Idx>,
    mask: B,
}
impl<B: BitSet> StackSet<B> {
    pub fn new(n: usize) -> Option<Self> {
        B::new(n).map(|mask| {
            StackSet{ stack: Vec::new(), mask: mask }
        })
    }
    pub fn push<F>(&mut self, i: B::Idx, f: F) where F: FnOnce(&mut Self) {
        if !self.mask.contains(i) {
            self.stack.push(i);
            self.mask.set(i);
            f(self);
            let _i = self.stack.pop().unwrap();
            self.mask.clear(i);
        }
    }
    pub fn is_full(&self) -> bool {
        self.mask.is_full()
    }
}

impl<B: BitSet> Deref for StackSet<B> {
    type Target = [B::Idx];
    fn deref(&self) -> &[B::Idx] {
        &self.stack
    }
}

macro_rules! make_impls { { $($ty:ty),* } => {
    $(impl BitSet for $ty {
        type Idx = u8;
        #[inline]
        fn new(n: usize) -> Option<Self> {
            let bits = size_of::<Self>() * 8;
            if n > bits {
                None
            } else if n == bits {
                Some(0)
            } else {
                Some(!0 << n)
            }
        }
        #[inline]
        fn is_full(&self) -> bool {
            !*self == 0
        }
        #[inline]
        fn contains(&self, i: u8) -> bool {
            *self & 1 << i != 0
        }
        #[inline]
        fn set(&mut self, i: u8) {
            *self |= 1 << i
        }
        #[inline]
        fn clear(&mut self, i: u8) {
            *self &= !(1 << i)
        }
    })*
}}
make_impls!{u8, u16, u32, usize, u64}

#[cfg(test)]
mod tests {
    use super::StackSet;

    type S32 = StackSet<u32>;
    
    #[test]
    fn empty() {
        let s = S32::new(0).unwrap();
        assert!(s.is_full());
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn size1() {
        let mut s = S32::new(1).unwrap();
        assert!(!s.is_full());
        assert_eq!(s.len(), 0);
        let mut called = false;
        s.push(0, |s| {
            called = true;
            assert!(s.is_full());
            assert_eq!(s.len(), 1);
            assert_eq!(s[0], 0);
        });
        assert!(called);
        assert!(!s.is_full());
        assert_eq!(s.len(), 0);
        called = false;
        s.push(0, |_s| called = true);
        assert!(called);
    }
}
