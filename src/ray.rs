use math::{Vec3f, vec3s};

pub struct Ray {
    pub org: Vec3f,
    pub dir: Vec3f,
    pub tmin: f32,
}

pub struct Isect {
    pub dist: f32,
    pub mat_id: int,
    pub light_id: int,
    pub normal: Vec3f,
}

impl Isect {
    pub fn new() -> Isect {
        Isect {
            dist: 1e36,
            mat_id: -1,
            light_id: -1,
            normal: vec3s(0.0),
        }
    }
}
