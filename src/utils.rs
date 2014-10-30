use math::Vec3f;

pub fn luminance(rgb: Vec3f) -> f32 {
    0.212671 * rgb.x +
        0.715160 * rgb.y +
        0.072169 * rgb.z
}
