mod ray;
mod vec3;

use ray::Ray;
use std::fs;
use vec3::Vec3;

fn color(r: Ray) -> Vec3 {
    let unit_direction = r.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let width = 200;
    let height = 100;
    let max_color = 255;

    let mut pic = format!("P3\n{} {}\n{}\n", width, height, max_color);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for h in (0..height).rev() {
        for w in 0..width {
            let u = w as f64 / width as f64;
            let v = h as f64 / height as f64;
            let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let col = color(r);
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
