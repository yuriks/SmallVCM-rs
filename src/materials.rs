use math::{Vec3f, vec3s};

pub struct Material {
    pub diffuse_reflectance: Vec3f,
    pub phong_reflectance: Vec3f,
    pub phong_exponent: f32,
    pub mirror_reflectance: Vec3f,
    pub ior: f32,
}

impl Material {
    pub fn new() -> Material {
        Material {
            diffuse_reflectance: vec3s(0.0),
            phong_reflectance: vec3s(0.0),
            phong_exponent: 1.0,
            mirror_reflectance: vec3s(0.0),
            ior: -1.0,
        }
    }
}
