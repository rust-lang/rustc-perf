//! Pseudo random number generators.
//!
//! Currently only `LFSR113` is implemented, since that is what ISPC uses, and it
//! allows us to compare Rust's codegen with that of ISPC for the same
//! algorithms.
//!
//! Use `{scalar,vector}::thread_rng()` to get a handle to the thread-local
//! random number generator, and call `.gen()` to generate an `f32` or an
//! `f32xN`.

/// Scalar pseudo random number generator
pub mod scalar {
    use std::cell::UnsafeCell;
    use std::rc::Rc;

    // Note: This implementation could be vectorized using an `u32x4`.
    struct RngT(u32, u32, u32, u32);

    impl RngT {
        fn from_seed(x: u32) -> Self {
            let z0 = x;
            let z1 = x ^ 0xbeeff00d;
            let z2 = ((x & 0xffff_u32) << 16) | (x >> 16);
            let z3 = ((x & 0xff_u32) << 24)
                | ((x & 0xff00_u32) << 8)
                | ((x & 0xff0000_u32) >> 8)
                | (x & 0xff000000_u32) >> 24;
            RngT(z0, z1, z2, z3)
        }

        pub fn gen_u32(&mut self) -> u32 {
            let mut b = ((self.0 << 6) ^ self.0) >> 13;
            self.0 = ((self.0 & 4294967294_u32) << 18) ^ b;
            b = ((self.1 << 2) ^ self.1) >> 27;
            self.1 = ((self.1 & 4294967288_u32) << 2) ^ b;
            b = ((self.2 << 13) ^ self.2) >> 21;
            self.2 = ((self.2 & 4294967280_u32) << 7) ^ b;
            b = ((self.3 << 3) ^ self.3) >> 12;
            self.3 = ((self.3 & 4294967168_u32) << 13) ^ b;
            self.0 ^ self.1 ^ self.2 ^ self.3
        }

        pub fn gen(&mut self) -> f32 {
            let mut v = self.gen_u32();
            v &= (1_u32 << 23) - 1;
            let v: f32 = unsafe { ::std::mem::transmute(0x3F800000 | v) };
            v - 1.
        }
    }

    #[derive(Clone)]
    pub struct RngH {
        rng: Rc<UnsafeCell<RngT>>,
    }

    impl RngH {
        pub fn gen(&mut self) -> f32 {
            unsafe { (*self.rng.get()).gen() }
        }
    }

    thread_local!(
        static THREAD_RNG_KEY: Rc<UnsafeCell<RngT>> = {
            Rc::new(UnsafeCell::new(RngT::from_seed(1)))
        }
    );

    pub fn thread_rng() -> RngH {
        RngH {
            rng: THREAD_RNG_KEY.with(|t| t.clone()),
        }
    }
}

/// Vector pseudo random number generator
pub mod vector {
    use geometry::{f32xN, u32xN, IncrV};
    use std::cell::UnsafeCell;
    use std::rc::Rc;
    struct RngT(u32xN, u32xN, u32xN, u32xN);

    impl RngT {
        fn from_seed(x: u32xN) -> Self {
            let z0 = x;
            let z1 = x ^ u32xN::splat(0xbeeff00d);
            let z2 = ((x & u32xN::splat(0xffff)) << 16) | (x >> 16);
            let z3 = ((x & u32xN::splat(0xff)) << 24)
                | ((x & u32xN::splat(0xff00)) << 8)
                | ((x & u32xN::splat(0xff0000)) >> 8)
                | (x & u32xN::splat(0xff000000)) >> 24;
            RngT(z0, z1, z2, z3)
        }

        pub fn gen_u32(&mut self) -> u32xN {
            let mut b = ((self.0 << 6) ^ self.0) >> 13;
            self.0 = ((self.0 & u32xN::splat(4294967294)) << 18) ^ b;
            b = ((self.1 << 2) ^ self.1) >> 27;
            self.1 = ((self.1 & u32xN::splat(4294967288)) << 2) ^ b;
            b = ((self.2 << 13) ^ self.2) >> 21;
            self.2 = ((self.2 & u32xN::splat(4294967280)) << 7) ^ b;
            b = ((self.3 << 3) ^ self.3) >> 12;
            self.3 = ((self.3 & u32xN::splat(4294967168)) << 13) ^ b;
            self.0 ^ self.1 ^ self.2 ^ self.3
        }

        pub fn gen(&mut self) -> f32xN {
            let mut v = self.gen_u32();
            v &= u32xN::splat((1_u32 << 23) - 1);
            let v: f32xN =
                unsafe { ::std::mem::transmute(u32xN::splat(0x3F800000) | v) };
            v - f32xN::splat(1.)
        }
    }

    #[derive(Clone)]
    pub struct RngH {
        rng: Rc<UnsafeCell<RngT>>,
    }

    impl RngH {
        pub fn gen(&mut self) -> f32xN {
            unsafe { (*self.rng.get()).gen() }
        }
    }

    thread_local!(
        static THREAD_RNG_KEY: Rc<UnsafeCell<RngT>> = {
            Rc::new(UnsafeCell::new(RngT::from_seed(<u32xN as IncrV>::incr(1))))
        }
    );

    pub fn thread_rng() -> RngH {
        RngH {
            rng: THREAD_RNG_KEY.with(|t| t.clone()),
        }
    }
}
