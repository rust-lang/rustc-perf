use crate::materials::Material;
use crate::vec::{Ray, Vec3};

#[derive(Clone, Copy)]
pub struct Hit<'obj> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'obj dyn Material,
}

pub trait Model {
    fn hit(&self, r: &Ray) -> Option<Hit>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

/// Minimum distance a ray must travel before we'll consider a possible hit.
///
/// If we try to use 0 here, we get a really strange bug. When a ray hits an object
/// and bounces, we'll sometimes register another hit on the same sphere,
/// at some tiny but positive distance, due to floating-point error.
///
const T_MIN: f32 = 0.0001;

impl Model for Sphere {
    fn hit<'a>(&'a self, r: &Ray) -> Option<Hit<'a>> {
        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let hb = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = hb * hb - a * c;
        if discriminant > 0.0 {
            let t = (-hb - discriminant.sqrt()) / a;
            if t >= T_MIN {
                let p = r.point_at_parameter(t);
                return Some(Hit {
                    t: t,
                    p: p,
                    normal: (p - self.center) / self.radius,
                    material: &*self.material,
                });
            }
            let t = (-hb + discriminant.sqrt()) / a;
            if t >= T_MIN {
                let p = r.point_at_parameter(t);
                return Some(Hit {
                    t: t,
                    p: p,
                    normal: (p - self.center) / self.radius,
                    material: &*self.material,
                });
            }
        }
        None
    }
}

impl Model for Vec<Box<dyn Model>> {
    fn hit(&self, r: &Ray) -> Option<Hit> {
        let mut best = None;
        for child in self {
            if let Some(hit) = child.hit(r) {
                match best {
                    None => best = Some(hit),
                    Some(prev) => {
                        if hit.t < prev.t {
                            best = Some(hit)
                        }
                    }
                }
            }
        }
        best
    }
}
