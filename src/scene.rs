use std::collections::SmallIntMap;

bitflags! {
    flags BoxMask: u32 {
        const LIGHT_CEILING    = 1,
        const LIGHT_SUN        = 2,
        const LIGHT_POINT      = 4,
        const LIGHT_BACKGROUND = 8,

        const LARGE_MIRROR_SPHERE = 16,
        const LARGE_GLASS_SPHERE  = 32,
        const SMALL_MIRROR_SPHERE = 64,
        const SMALL_GLASS_SPHERE  = 128,

        const GLOSSY_FLOOR = 256,

        const BOTH_SMALL_SPHERES = SMALL_MIRROR_SPHERE.bits | SMALL_GLASS_SPHERE.bits,
        const BOTH_LARGE_SPHERES = LARGE_MIRROR_SPHERE.bits | LARGE_GLASS_SPHERE.bits,
        const DEFAULT            = LIGHT_CEILING.bits | BOTH_SMALL_SPHERES.bits,
    }
}

pub struct Scene {
    geometry: (), // TODO
    camera: (), // TODO
    materials: Vec<()>, // TODO
    lights: Vec<()>, // TODO
    material_to_light: SmallIntMap<uint>,
    scene_sphere: (), // TODO
    background: (), // TODO

    scene_name: String,
    scene_acronym: String,
}
