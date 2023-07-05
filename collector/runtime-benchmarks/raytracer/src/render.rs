use lodepng::RGB;

use crate::camera::Camera;
use crate::model::Model;
use crate::vec::{Ray, Vec3};
use std::f32::consts::PI;

fn color(mut r: Ray, model: &dyn Model) -> Vec3 {
    const WHITE: Vec3 = Vec3(1.0, 1.0, 1.0);
    let sky_blue = 0.3 * Vec3(0.5, 0.7, 1.0) + 0.7 * WHITE;

    let mut attenuation = WHITE;
    let mut depth = 0;
    while let Some(hit) = model.hit(&r) {
        let scattered = hit.material.scatter(&r, &hit);
        attenuation = attenuation * scattered.color;
        if let Some(bounce) = scattered.ray {
            r = bounce;
        } else {
            break;
        }

        depth += 1;
        if depth >= 50 {
            break;
        }
    }
    let sun_direction = Vec3(1.0, 1.0, 1.0).to_unit_vector();
    let unit_direction = r.direction.to_unit_vector();
    if sun_direction.dot(unit_direction) >= (5.0 * PI / 180.0).cos() {
        Vec3(5.0, 5.0, 3.0) * attenuation // SUPER BRIGHT
    } else {
        let t = 0.5 * (unit_direction.y() + 1.0);
        let orig_color = (1.0 - t) * WHITE + t * sky_blue;
        orig_color * attenuation
    }
}

pub fn render(
    scene: &dyn Model,
    camera: &Camera,
    width: usize,
    height: usize,
    samples: usize,
) -> Vec<RGB<u8>> {
    let mut pixels: Vec<RGB<u8>> = Vec::with_capacity(width * height);
    for y in 0..height {
        let j = height - 1 - y;
        for i in 0..width {
            let mut col = Vec3(0.0, 0.0, 0.0);
            for _ in 0..samples {
                let u = (i as f32 + 0.5) / width as f32;
                let v = (j as f32 + 0.5) / height as f32;

                let r = camera.get_ray(u, v);
                col = col + color(r, scene);
            }
            col = col / samples as f32;
            col = Vec3(col.x().sqrt(), col.y().sqrt(), col.z().sqrt());
            let rgb = col.to_u8();
            pixels.push(RGB {
                r: rgb[0],
                g: rgb[1],
                b: rgb[2],
            });
        }
    }
    pixels
}
