/// Scene interface
use geometry::{f32xN, Plane, Sphere};

pub trait Scene: Send + Sync {
    const NAO_SAMPLES: usize;
    fn new() -> Self;
    fn rand(&mut self) -> f32;
    fn plane(&self) -> &Plane;
    fn spheres(&self) -> &[Sphere];
    fn rand_f32xN(&mut self) -> (f32xN, f32xN) {
        #[cfg(feature = "256bit")]
        {
            let r = [
                self.rand(),
                self.rand(),
                self.rand(),
                self.rand(),
                self.rand(),
                self.rand(),
                self.rand(),
                self.rand(),
                self.rand(),
                self.rand(),
                self.rand(),
                self.rand(),
                self.rand(),
                self.rand(),
                self.rand(),
                self.rand(),
            ];
            (
                f32xN::new(r[0], r[2], r[4], r[6], r[8], r[10], r[12], r[14]),
                f32xN::new(r[1], r[3], r[5], r[7], r[9], r[11], r[13], r[15]),
            )
        }
        #[cfg(not(feature = "256bit"))]
        {
            let r = [
                self.rand(),
                self.rand(),
                self.rand(),
                self.rand(),
                self.rand(),
                self.rand(),
                self.rand(),
                self.rand(),
            ];
            (
                f32xN::new(r[0], r[2], r[4], r[6]),
                f32xN::new(r[1], r[3], r[5], r[7]),
            )
        }
    }
}

mod random;
pub use self::random::Random;

mod test;
pub use self::test::Test;
