use benchlib::benchmark::run_benchmark_group;
use camera::Camera;
use materials::{Dielectric, Lambertian, Material, Metal};
use model::{Model, Sphere};
use vec::{random_in_unit_disc, Vec3};

mod camera; // translate 2D pixel coordinates to 3D rays
mod materials; // reflective properties of surfaces
mod model; // geometry of objects in the scene
mod render;
mod vec; // basic 3D vector math // the core ray-tracing algorithm

/// Generate a Model containing a bunch of randomly placed spheres.
fn create_scene() -> Box<dyn Model> {
    let mut spheres: Vec<Sphere> = vec![
        Sphere {
            center: Vec3(0.0, 0.0, -1000.0),
            radius: 1000.0,
            material: Box::new(Lambertian {
                albedo: Vec3(1.0, 0.6, 0.5),
            }),
        },
        Sphere {
            center: Vec3(-4.0, 0.0, 2.0),
            radius: 2.0,
            material: Box::new(Lambertian {
                albedo: Vec3(0.6, 0.2, 0.2),
            }),
        },
        Sphere {
            center: Vec3(0.0, 0.0, 2.0),
            radius: 2.0,
            material: Box::new(Dielectric { index: 1.5 }),
        },
        Sphere {
            center: Vec3(4.0, 0.0, 2.0),
            radius: 2.0,
            material: Box::new(Metal {
                albedo: Vec3(0.85, 0.9, 0.7),
                fuzz: 0.0,
            }),
        },
    ];

    fn random_material() -> Box<dyn Material> {
        Box::new(Lambertian {
            albedo: Vec3(0.5, 0.5, 0.5),
        })
    }

    for _ in 0..500 {
        let r = 0.4;
        let Vec3(x, y, _) = random_in_unit_disc();
        let pos = 20.0 * Vec3(x, y, 0.0) + Vec3(0.0, 0.0, r);
        if spheres
            .iter()
            .all(|s| (s.center - pos).length() >= s.radius + r)
        {
            spheres.push(Sphere {
                center: pos,
                radius: r,
                material: random_material(),
            });
        }
    }

    let world: Vec<Box<dyn Model>> = spheres
        .into_iter()
        .map(|s| Box::new(s) as Box<dyn Model>)
        .collect();
    Box::new(world)
}

fn main() {
    const WIDTH: usize = 400;
    const HEIGHT: usize = 200;

    const NSAMPLES: usize = 100;

    let scene = create_scene();
    let lookfrom = Vec3(20.0 * 0.47f32.cos(), 20.0 * 0.47f32.sin(), 3.0);
    let lookat = Vec3(0.0, 0.0, 1.0);
    let vup = Vec3(0.0, 0.0, 1.0);
    let focus_distance = (lookfrom - lookat).length();
    let aperture = 0.3;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        WIDTH as f32 / HEIGHT as f32,
        aperture,
        focus_distance,
    );

    run_benchmark_group(|group| {
        // Performs raytracing on a simple scene.
        // Adapted from https://github.com/jorendorff/rust-raytrace.
        group.register_benchmark("raytracer", || {
            || render::render(scene.as_ref(), &camera, WIDTH, HEIGHT, NSAMPLES)
        });
    });
}
