use crate::model::Hit;
use crate::vec::{random_in_unit_sphere, Ray, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Scatter {
    pub color: Vec3,
    pub ray: Option<Ray>,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &Hit) -> Scatter;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, hit: &Hit) -> Scatter {
        let target = hit.p + hit.normal + random_in_unit_sphere();
        Scatter {
            color: self.albedo,
            ray: Some(Ray::new(hit.p, target - hit.p)),
        }
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Scatter {
        let reflected = reflect(r_in.direction, hit.normal);
        let scattered = Ray::new(hit.p, reflected + self.fuzz * random_in_unit_sphere());

        Scatter {
            color: self.albedo,
            ray: if scattered.direction.dot(hit.normal) <= 0.0 {
                None
            } else {
                Some(scattered)
            },
        }
    }
}

pub struct Dielectric {
    // Technically, this is not the index of refaction but the ratio of the
    // index of refraction inside the material to the index of refraction
    // outside.  But if the material outside is air, its index of refraction is
    // 1 and so it amounts to the same thing.
    pub index: f32,
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.to_unit_vector();

    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - dt * n) - discriminant.sqrt() * n)
    } else {
        None
    }
}

const WHITE: Vec3 = Vec3(1.0, 1.0, 1.0);

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Scatter {
        let outward_normal: Vec3;
        let ni_over_nt: f32;

        if r_in.direction.dot(hit.normal) > 0.0 {
            outward_normal = -hit.normal;
            ni_over_nt = self.index;
        } else {
            outward_normal = hit.normal;
            ni_over_nt = 1.0 / self.index;
        }

        let _ = refract(r_in.direction, outward_normal, ni_over_nt);

        Scatter {
            color: WHITE,
            ray: Some(Ray::new(hit.p, reflect(r_in.direction, hit.normal))),
        }
    }
}
