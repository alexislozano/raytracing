use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    t: f64,
    p: Vec3,
    normal: Vec3,
}

impl HitRecord {
    pub fn new(t: f64, p: Vec3, normal: Vec3) -> HitRecord {
        HitRecord { t, p, normal }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn p(&self) -> Vec3 {
        self.p
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center();
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius() * self.radius();
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let t1 = (-b - discriminant.sqrt()) / a;
            let t2 = (-b + discriminant.sqrt()) / a;
            if t1 < t_max && t1 > t_min {
                let p = r.point_at_parameter(t1);
                let n = (p - self.center()) / self.radius();
                Some(HitRecord::new(t1, p, n))
            } else if t2 < t_max && t2 > t_min {
                let p = r.point_at_parameter(t2);
                let n = (p - self.center()) / self.radius();
                Some(HitRecord::new(t2, p, n))
            } else {
                None
            }
        } else {
            None
        }
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
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut res: Option<HitRecord> = None;
        for h in self.list.iter() {
            let new_res = h.hit(r, t_min, closest_so_far);
            match new_res {
                Some(hit_record) => {
                    closest_so_far = hit_record.t();
                    res = Some(HitRecord::new(
                        hit_record.t(),
                        hit_record.p(),
                        hit_record.normal(),
                    ));
                }
                None => (),
            };
        }
        res
    }
}
