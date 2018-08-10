//! Aobench scene: 3 spheres and a plane using a random number generator

use geometry::{f32xN, Plane, Sphere, V3D};
use scene::Scene;

#[derive(Clone)]
pub struct Random {
    pub plane: Plane,
    pub spheres: [Sphere; 3],
}

impl Scene for Random {
    const NAO_SAMPLES: usize = 8;
    fn new() -> Self {
        let plane = Plane {
            p: V3D {
                x: 0.,
                y: -0.5,
                z: 0.,
            },
            n: V3D {
                x: 0.,
                y: 1.,
                z: 0.,
            },
        };
        let spheres = [
            Sphere {
                center: V3D {
                    x: -2.,
                    y: 0.,
                    z: -3.5,
                },
                radius: 0.5,
            },
            Sphere {
                center: V3D {
                    x: -0.5,
                    y: 0.,
                    z: -3.,
                },
                radius: 0.5,
            },
            Sphere {
                center: V3D {
                    x: 1.,
                    y: 0.,
                    z: -2.2,
                },
                radius: 0.5,
            },
        ];
        Self { plane, spheres }
    }
    fn rand(&mut self) -> f32 {
        ::random::scalar::thread_rng().gen()
    }
    fn plane(&self) -> &Plane {
        &self.plane
    }
    fn spheres(&self) -> &[Sphere] {
        &self.spheres
    }
    fn rand_f32xN(&mut self) -> (f32xN, f32xN) {
        let mut rng = ::random::vector::thread_rng();
        (rng.gen(), rng.gen())
    }
}
