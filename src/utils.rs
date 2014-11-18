use math::{Vec3f, INV_PI};
use std::num::FloatMath;

pub fn luminance(rgb: Vec3f) -> f32 {
    0.212671 * rgb.x +
        0.715160 * rgb.y +
        0.072169 * rgb.z
}

pub fn cos_hemisphere_pdf_w(normal: &Vec3f, direction: Vec3f) -> f32 {
    normal.dot(direction).max(0.0) * INV_PI
}

pub fn concentric_disc_pdf_a() -> f32 {
    INV_PI
}

pub fn uniform_sphere_pdf_w() -> f32 {
    INV_PI * 0.25
}
