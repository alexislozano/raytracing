use crate::vec3::Vec3;

pub enum Material {
    Lambertian { attenuation: Vec3 },
    Metal { attenuation: Vec3, fuzziness: f64 },
}
