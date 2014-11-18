use math::{Vec3f, vec3, vec3s};
use frame::Frame;
use utils::{cos_hemisphere_pdf_w, concentric_disc_pdf_a, uniform_sphere_pdf_w};
use std::num::FloatMath;

pub struct SceneSphere {
    pub scene_center: Vec3f,
    pub scene_radius: f32,
    pub inv_scene_radius_sqr: f32,
}

pub trait AbstractLight {
    fn get_radiance(&self, _: &SceneSphere, ray_direction: &Vec3f, hit_point: &Vec3f,
                    direct_pdf_a: Option<&mut f32>, emission_pdf_w: Option<&mut f32>) -> Vec3f;
    fn is_finite() -> bool;
    fn is_delta() -> bool;
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
    fn get_radiance(&self, _: &SceneSphere, ray_direction: &Vec3f, _hit_point: &Vec3f,
                    direct_pdf_a: Option<&mut f32>, emission_pdf_w: Option<&mut f32>) -> Vec3f {
        let cos_out_l = self.frame.normal().dot(-*ray_direction).max(0.0);

        if cos_out_l == 0.0 {
            return vec3s(0.0);
        }

        if let Some(direct_pdf_a) = direct_pdf_a {
            *direct_pdf_a = self.inv_area;
        }

        if let Some(emission_pdf_w) = emission_pdf_w {
            *emission_pdf_w = cos_hemisphere_pdf_w(&self.frame.normal(), -*ray_direction);
            *emission_pdf_w *= self.inv_area;
        }

        self.intensity
    }

    fn is_finite() -> bool { true }
    fn is_delta() -> bool { false }
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
    fn get_radiance(&self, _: &SceneSphere, _ray_direction: &Vec3f, _hit_point: &Vec3f,
                    _direct_pdf_a: Option<&mut f32>, _emission_pdf_w: Option<&mut f32>) -> Vec3f {
        vec3s(0.0)
    }

    fn is_finite() -> bool { false }
    fn is_delta() -> bool { true }
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
    fn get_radiance(&self, _: &SceneSphere, _ray_direction: &Vec3f, _hit_point: &Vec3f,
                    _direct_pdf_a: Option<&mut f32>, _emission_pdf_w: Option<&mut f32>) -> Vec3f {
        vec3s(0.0)
    }

    fn is_finite() -> bool { true }
    fn is_delta() -> bool { true }
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
    fn get_radiance(&self, scene_sphere: &SceneSphere, _ray_direction: &Vec3f, _hit_point: &Vec3f,
                    direct_pdf_a: Option<&mut f32>, emission_pdf_w: Option<&mut f32>) -> Vec3f {
        let direct_pdf = uniform_sphere_pdf_w();
        let radiance = self.background_color * vec3s(self.scale);

        let position_pdf = concentric_disc_pdf_a() * scene_sphere.inv_scene_radius_sqr;

        if let Some(direct_pdf_a) = direct_pdf_a {
            *direct_pdf_a = direct_pdf;
        }

        if let Some(emission_pdf_w) = emission_pdf_w {
            *emission_pdf_w = direct_pdf * position_pdf;
        }

        radiance
    }

    fn is_finite() -> bool { false }
    fn is_delta() -> bool { false }
}
