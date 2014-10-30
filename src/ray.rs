use math::Vec3f;

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
