use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
}

impl HitRecord {
    fn new(t: f64, p: Vec3, normal: Vec3) -> HitRecord {
        HitRecord { t, p, normal }
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> (bool, HitRecord);
}

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> (bool, HitRecord) {
        let mut rec = HitRecord::new(t_max, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let mut t = (-b - discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                let p = r.point_at_parameter(t);
                let n = (p - self.center) / self.radius;
                rec.t = t;
                rec.p = p;
                rec.normal = n;
                return (true, rec);
            }
            t = (-b + discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                let p = r.point_at_parameter(t);
                let n = (p - self.center) / self.radius;
                rec.t = t;
                rec.p = p;
                rec.normal = n;
                return (true, rec);
            }
        }
        (false, rec)
    }
}

pub struct HitableList {
    list: Vec<Box<Hitable>>,
}

impl HitableList {
    pub fn new(list: Vec<Box<Hitable>>) -> HitableList {
        HitableList { list }
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> (bool, HitRecord) {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut rec = HitRecord::new(
            closest_so_far,
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
        );
        for h in self.list.iter() {
            let res = h.hit(r, t_min, closest_so_far);
            if res.0 {
                hit_anything = true;
                closest_so_far = res.1.t;
                rec = res.1;
            }
        }
        (hit_anything, rec)
    }
}
