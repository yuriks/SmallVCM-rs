use std::collections::SmallIntMap;
use math::{Vec2i, vec2, vec3};
use camera::Camera;

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
    camera: Camera,
    materials: Vec<()>, // TODO
    lights: Vec<()>, // TODO
    material_to_light: SmallIntMap<uint>,
    scene_sphere: (), // TODO
    background: (), // TODO

    pub scene_name: String,
    pub scene_acronym: String,
}

impl Scene {
    fn load_cornell_box(resolution: Vec2i, mut box_mask: BoxMask) -> Scene {
        let (name, acronym) = Scene::get_scene_name(box_mask);

        if (box_mask & BOTH_LARGE_SPHERES) == BOTH_LARGE_SPHERES {
            println!("Cannot have both large balls, using mirror\n");
            box_mask.remove(LARGE_GLASS_SPHERE);
        }

        let light_ceiling    = box_mask.contains(LIGHT_CEILING);
        let light_sun        = box_mask.contains(LIGHT_SUN);
        let light_point      = box_mask.contains(LIGHT_POINT);
        let light_background = box_mask.contains(LIGHT_BACKGROUND);

        let light_box = !light_point;


        // TODO
        Scene {
            geometry: (),
            camera: Camera::new(
                vec3(-0.0439815, -4.12529,   0.222539),
                vec3( 0.00688625, 0.998505, -0.0542161),
                vec3( 3.73896e-4, 0.0542148, 0.998529),
                vec2(resolution.x as f32, resolution.y as f32), 45.0),
            materials: Vec::new(),
            lights: Vec::new(),
            material_to_light: SmallIntMap::new(),
            scene_sphere: (),
            background: (),
            scene_name: name,
            scene_acronym: acronym,
        }
    }

    fn get_scene_name(box_mask: BoxMask) -> (String, String) {
        let mut name = String::new();
        let mut acronym = String::new();

        // TODO
        name.push_str("scene name");
        acronym.push_str("XXX");

        (name, acronym)
    }
}
