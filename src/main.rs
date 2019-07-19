mod camera;
mod hitable;
mod ray;
mod vec3;

use camera::Camera;
use hitable::{Hitable, HitableList, Sphere};
use ray::Ray;
use std::fs;
use vec3::Vec3;

use rand::prelude::*;

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p = Vec3::new(1.0, 1.0, 1.0);
    while p.squared_length() >= 1.0 {
        p = Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) * 2.0
            - Vec3::new(1.0, 1.0, 1.0);
    }
    p
}

fn color(r: &Ray, world: &HitableList) -> Vec3 {
    let res = world.hit(r, 0.001, std::f64::INFINITY);
    match res {
        Some(hitable_record) => {
            let n = hitable_record.normal();
            let p = hitable_record.p();
            let target = p + n + random_in_unit_sphere();
            color(&Ray::new(p, target - p), world) * 0.5
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

    let hitable1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let hitable2 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);

    let world = HitableList::new(vec![Box::new(hitable1), Box::new(hitable2)]);

    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        Vec3::new(-2.0, -1.0, -1.0),
    );

    for h in (0..height).rev() {
        for w in 0..width {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ray_per_pixel {
                let u = (w as f64 + rng.gen::<f64>()) / width as f64;
                let v = (h as f64 + rng.gen::<f64>()) / height as f64;
                let r = &camera.get_ray(u, v);
                col = col + color(&r, &world);
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
