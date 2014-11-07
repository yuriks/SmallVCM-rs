use math::{Vec3f, vec3, vec3s};
use frame::Frame;

pub struct SceneSphere {
    pub scene_center: Vec3f,
    pub scene_radius: f32,
    pub inv_scene_radius_sqr: f32,
}

pub trait AbstractLight {
    // TODO
}

pub struct AreaLight {
    p0: Vec3f,
    e1: Vec3f,
    e2: Vec3f,
    frame: Frame,
    pub intensity: Vec3f,
    inv_area: f32,
}

impl AreaLight {
    pub fn new(p0: Vec3f, p1: Vec3f, p2: Vec3f) -> AreaLight {
        let e1 = p1 - p0;
        let e2 = p2 - p0;
        let normal = e1.cross(e2);
        let len = normal.length();
        AreaLight {
            p0: p0,
            e1: e1,
            e2: e2,
            frame: Frame::identity(),
            intensity: vec3s(0.0),
            inv_area: 2.0 / len,
        }
    }
}

impl AbstractLight for AreaLight {
    // TODO
}

pub struct DirectionalLight {
    frame: Frame,
    pub intensity: Vec3f,
}

impl DirectionalLight {
    pub fn new(direction: Vec3f) -> DirectionalLight {
        DirectionalLight {
            frame: Frame::from_z(direction),
            intensity: vec3s(0.0),
        }
    }
}

impl AbstractLight for DirectionalLight {
    // TODO
}

pub struct PointLight {
    position: Vec3f,
    pub intensity: Vec3f,
}

impl PointLight {
    pub fn new(position: Vec3f) -> PointLight {
        PointLight {
            position: position,
            intensity: vec3s(0.0),
        }
    }
}

impl AbstractLight for PointLight {
    // TODO
}

pub struct BackgroundLight {
    background_color: Vec3f,
    pub scale: f32,
}

impl BackgroundLight {
    pub fn new() -> BackgroundLight {
        BackgroundLight {
            background_color: vec3(135.0, 206.0, 250.0) / vec3s(255.0),
            scale: 1.0,
        }
    }
}

impl AbstractLight for BackgroundLight {
    // TODO
}
