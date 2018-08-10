//! Aobench scene: 3 spheres and a plane using a random number generator

use geometry::{Plane, Sphere, V3D};
use scene::Scene;
use std::num::Wrapping;

#[derive(Clone)]
pub struct Test {
    pub plane: Plane,
    pub spheres: [Sphere; 3],
    rands: Vec<f32>,
    rand_step: Wrapping<usize>,
}

impl Scene for Test {
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
        let mut rands = Vec::new();
        let mut rng = ::random::scalar::thread_rng();
        for _ in 0..2 * Self::NAO_SAMPLES * Self::NAO_SAMPLES {
            rands.push(rng.gen());
        }
        let rand_step = Wrapping(0);
        Self {
            plane,
            spheres,
            rands,
            rand_step,
        }
    }
    fn rand(&mut self) -> f32 {
        let v = self.rands[self.rand_step.0];
        self.rand_step += Wrapping(1);
        if self.rand_step
            >= Wrapping(2 * Self::NAO_SAMPLES * Self::NAO_SAMPLES)
        {
            self.rand_step = Wrapping(0);
        }
        v
    }
    fn plane(&self) -> &Plane {
        &self.plane
    }
    fn spheres(&self) -> &[Sphere] {
        &self.spheres
    }
}
