use math::Vec2i;
use math::vec2;
use std::default::Default;

enum Algorithm {
    EyeLight
}

impl Algorithm {
    fn get_name(self) -> &'static str {
        match self {
            EyeLight => "eye light",
        }
    }

    fn get_acronym(self) -> &'static str {
        match self {
            EyeLight => "el",
        }
    }
}

pub struct Config {
    scene: (), // TODO
    algorithm: Algorithm,
    iterations: u32,
    max_time: f32,
    radius_factor: f32,
    radius_alpha: f32,
    framebuffer: (), // TODO
    pub num_threads: u32,
    base_seed: u32,
    max_path_length: uint,
    min_path_length: uint,
    output_name: String,
    resolution: Vec2i,
    full_report: bool,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            scene: (), // TODO
            algorithm: EyeLight, // TODO
            iterations: 1,
            max_time: -1.0,
            radius_factor: 0.003,
            radius_alpha: 0.75,
            framebuffer: (), // TODO
            num_threads: 0,
            base_seed: 1234,
            max_path_length: 10,
            min_path_length: 0,
            output_name: "".to_string(),
            resolution: vec2(512, 512),
            full_report: false,
        }
    }
}
