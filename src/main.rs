mod camera;
mod hitable;
mod material;
mod ray;
mod vec3;

use camera::Camera;
use hitable::{Hitable, HitableList, Sphere};
use material::Material;
use ray::Ray;
use std::fs;
use vec3::Vec3;

use rand::prelude::*;

fn color(r: &Ray, world: &HitableList, depth: u32) -> Vec3 {
    let (hit_record, material) = world.hit(r, 0.001, std::f64::INFINITY);
    match hit_record {
        Some(hit_record) => {
            let n = hit_record.normal();
            let p = hit_record.p();
            match material {
                Some(material) => {
                    let (scattered, attenuation, b) = material.scatter(r, n, p);
                    if depth < 50 && b {
                        attenuation * color(&scattered, world, depth + 1)
                    } else {
                        Vec3::new(0.0, 0.0, 0.0)
                    }
                }
                None => Vec3::new(0.0, 0.0, 0.0),
            }
        }
        None => {
            let unit_direction = r.direction().unit();
            let t = 0.5 * (unit_direction.y + 1.0);
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    let width = 200;
    let height = 100;
    let ray_per_pixel = 100;
    let max_color = 255;

    let mut pic = format!("P3\n{} {}\n{}\n", width, height, max_color);

    let material1 = Material::Lambertian {
        attenuation: Vec3::new(0.1, 0.2, 0.5),
    };
    let material2 = Material::Lambertian {
        attenuation: Vec3::new(0.8, 0.8, 0.0),
    };
    let material3 = Material::Metal {
        attenuation: Vec3::new(0.8, 0.6, 0.2),
        fuzziness: 0.3,
    };
    let material4 = Material::Dielectric {
        refraction: 1.5,
    };
    let material5 = Material::Dielectric {
        refraction: 1.5,
    };

    let hitable1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material1);
    let hitable2 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material2);
    let hitable3 = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material3);
    let hitable4 = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material4);
    let hitable5 = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45, material5);

    let world = HitableList::new(vec![
        Box::new(hitable1),
        Box::new(hitable2),
        Box::new(hitable3),
        Box::new(hitable4),
        Box::new(hitable5),
    ]);

    let camera = Camera::new(
        Vec3::new(-2.0, 2.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0, 
        width as f64 / height as f64,
    );

    for h in (0..height).rev() {
        for w in 0..width {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ray_per_pixel {
                let u = (w as f64 + rng.gen::<f64>()) / width as f64;
                let v = (h as f64 + rng.gen::<f64>()) / height as f64;
                let r = &camera.get_ray(u, v);
                col = col + color(&r, &world, 0);
            }
            col = col / ray_per_pixel as f64;
            col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
            let ir = (max_color as f64 * col.x) as usize;
            let ig = (max_color as f64 * col.y) as usize;
            let ib = (max_color as f64 * col.z) as usize;
            pic = format!("{}{} {} {}\n", &pic, ir, ig, ib);
        }
    }

    match fs::write("output.ppm", pic) {
        Err(_) => eprintln!("Could not generate the picture"),
        Ok(_) => (),
    };
}
