use std::collections::SmallIntMap;
use math::{Vec2i, vec2, vec3, vec3s};
use camera::Camera;
use materials::Material;
use geometry::{GeometryList, Triangle, Sphere};

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
    geometry: GeometryList,
    camera: Camera,
    materials: Vec<Material>,
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

        let materials = vec![
            // 0) light1, will only emit
            Material::new(),
            // 1) light2, will only emit
            Material::new(),

            // 2) glossy white floor
            Material {
                diffuse_reflectance: vec3s(0.1),
                phong_reflectance: vec3s(0.7),
                phong_exponent: 90.0,
                ..Material::new()
            },

            // 3) diffuse green left wall
            Material {
                diffuse_reflectance: vec3(0.156863, 0.803922, 0.172549),
                ..Material::new()
            },

            // 4) diffuse red right wall
            Material {
                diffuse_reflectance: vec3(0.803922, 0.152941, 0.152941),
                ..Material::new()
            },

            // 5) diffuse white back wall
            Material {
                diffuse_reflectance: vec3s(0.803922),
                ..Material::new()
            },

            // 6) mirror ball
            Material {
                mirror_reflectance: vec3s(1.0),
                ..Material::new()
            },

            // 7) glass ball
            Material {
                mirror_reflectance: vec3s(1.0),
                ior: 1.6,
                ..Material::new()
            },

            // 8) diffuse blue wall (back wall for glossy floor)
            Material {
                diffuse_reflectance: vec3(0.156863, 0.172549, 0.803922),
                ..Material::new()
            }
        ];

        // Cornell box
        let cb = [
            vec3(-1.27029,  1.30455, -1.28002),
            vec3( 1.28975,  1.30455, -1.28002),
            vec3( 1.28975,  1.30455,  1.28002),
            vec3(-1.27029,  1.30455,  1.28002),
            vec3(-1.27029, -1.25549, -1.28002),
            vec3( 1.28975, -1.25549, -1.28002),
            vec3( 1.28975, -1.25549,  1.28002),
            vec3(-1.27029, -1.25549,  1.28002),
        ];

        let mut geometry_list = GeometryList::new();

        let (mat_a, mat_b) = if box_mask.contains(GLOSSY_FLOOR) { (2, 8) } else { (5, 5) };
        // Floor
        geometry_list.geometry.push(box Triangle::new(cb[0], cb[4], cb[5], mat_a));
        geometry_list.geometry.push(box Triangle::new(cb[5], cb[1], cb[0], mat_a));
        // Back wall
        geometry_list.geometry.push(box Triangle::new(cb[0], cb[1], cb[2], mat_b));
        geometry_list.geometry.push(box Triangle::new(cb[2], cb[3], cb[0], mat_b));

        // Ceiling
        let (mat_a, mat_b) = if light_ceiling && !light_box { (0, 1) } else { (5, 5) };
        geometry_list.geometry.push(box Triangle::new(cb[2], cb[6], cb[7], mat_a));
        geometry_list.geometry.push(box Triangle::new(cb[7], cb[3], cb[2], mat_b));

        // Left wall
        geometry_list.geometry.push(box Triangle::new(cb[3], cb[7], cb[4], 3));
        geometry_list.geometry.push(box Triangle::new(cb[4], cb[0], cb[3], 3));

        // Right wall
        geometry_list.geometry.push(box Triangle::new(cb[1], cb[5], cb[6], 4));
        geometry_list.geometry.push(box Triangle::new(cb[6], cb[2], cb[1], 4));

        // Ball - central
        let large_radius = 0.8;
        let center = (cb[0] + cb[1] + cb[4] + cb[5]) * vec3s(0.25) + vec3(0.0, 0.0, large_radius);

        if box_mask.contains(LARGE_MIRROR_SPHERE) {
            geometry_list.geometry.push(box Sphere::new(center, large_radius, 6));
        }

        if box_mask.contains(LARGE_GLASS_SPHERE) {
            geometry_list.geometry.push(box Sphere::new(center, large_radius, 7));
        }

        // Balls - left and right
        let small_radius = 0.5;
        let left_wall_center  = (cb[0] + cb[4]) * vec3s(0.5) + vec3(0.0, 0.0, small_radius);
        let right_wall_center = (cb[1] + cb[5]) * vec3s(0.5) + vec3(0.0, 0.0, small_radius);
        let xlen = right_wall_center.x - left_wall_center.x;
        let left_ball_center  = left_wall_center  + vec3(2.0 * xlen / 7.0, 0.0, 0.0);
        let right_ball_center = right_wall_center + vec3(2.0 * xlen / 7.0, 0.0, 0.0);

        if box_mask.contains(SMALL_MIRROR_SPHERE) {
            geometry_list.geometry.push(box Sphere::new(left_ball_center, small_radius, 6));
        }

        if box_mask.contains(SMALL_GLASS_SPHERE) {
            geometry_list.geometry.push(box Sphere::new(left_ball_center, small_radius, 7));
        }

        // Light box at the ceiling
        let lb = [
            vec3(-0.25,  0.25, 1.26002),
            vec3( 0.25,  0.25, 1.26002),
            vec3( 0.25,  0.25, 1.28002),
            vec3(-0.25,  0.25, 1.28002),
            vec3(-0.25, -0.25, 1.26002),
            vec3( 0.25, -0.25, 1.26002),
            vec3( 0.25, -0.25, 1.28002),
            vec3(-0.25, -0.25, 1.28002),
        ];

        if light_box {
            // Back wall
            geometry_list.geometry.push(box Triangle::new(lb[0], lb[2], lb[1], 5));
            geometry_list.geometry.push(box Triangle::new(lb[2], lb[0], lb[3], 5));
            // Left wall
            geometry_list.geometry.push(box Triangle::new(lb[3], lb[4], lb[7], 5));
            geometry_list.geometry.push(box Triangle::new(lb[4], lb[3], lb[0], 5));
            // Right wall
            geometry_list.geometry.push(box Triangle::new(lb[1], lb[6], lb[5], 5));
            geometry_list.geometry.push(box Triangle::new(lb[6], lb[1], lb[2], 5));
            // Front wall
            geometry_list.geometry.push(box Triangle::new(lb[4], lb[5], lb[6], 5));
            geometry_list.geometry.push(box Triangle::new(lb[6], lb[7], lb[4], 5));

            // Floor
            let (mat_a, mat_b) = if light_ceiling { (0, 1) } else { (5, 5) };
            geometry_list.geometry.push(box Triangle::new(lb[0], lb[5], lb[4], mat_a));
            geometry_list.geometry.push(box Triangle::new(lb[5], lb[0], lb[1], mat_b));
        }

        // TODO
        Scene {
            geometry: geometry_list,
            camera: Camera::new(
                vec3(-0.0439815, -4.12529,   0.222539),
                vec3( 0.00688625, 0.998505, -0.0542161),
                vec3( 3.73896e-4, 0.0542148, 0.998529),
                vec2(resolution.x as f32, resolution.y as f32), 45.0),
            materials: materials,
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
