use crate::ray::Ray;
use crate::vec3::Vec3;
use std::f64::consts::PI;

use rand::prelude::*;

fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p = Vec3::new(1.0, 1.0, 1.0);
    while p.dot(p) >= 1.0 {
        p = Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), 0.0) * 2.0 - Vec3::new(1.0, 1.0, 0.0);
    }
    p
}

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let lens_radius = aperture / 2.0;
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let origin = lookfrom;
        let w = (lookfrom - lookat).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);
        let lower_left_corner =
            origin - u * half_width * focus_dist - v * half_height * focus_dist - w * focus_dist;
        let horizontal = u * half_width * focus_dist * 2.0;
        let vertical = v * half_height * focus_dist * 2.0;
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            lens_radius,
        }
    }

    fn origin(&self) -> Vec3 {
        self.origin
    }

    fn horizontal(&self) -> Vec3 {
        self.horizontal
    }

    fn vertical(&self) -> Vec3 {
        self.vertical
    }

    fn lower_left_corner(&self) -> Vec3 {
        self.lower_left_corner
    }

    fn u(&self) -> Vec3 {
        self.u
    }

    fn v(&self) -> Vec3 {
        self.v
    }

    fn lens_radius(&self) -> f64 {
        self.lens_radius
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius();
        let offset = self.u() * rd.x() + self.v() * rd.y();
        Ray::new(
            self.origin() + offset,
            self.lower_left_corner() + self.horizontal() * s + self.vertical() * t
                - self.origin()
                - offset,
        )
    }
}
