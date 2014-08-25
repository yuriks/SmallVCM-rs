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

struct Config {
    scene: (), // TODO
    algorithm: Algorithm,
    iterations: u32,
    max_time: f32,
    radius_factor: f32,
    radius_alpha: f32,
    framebuffer: (), // TODO
    num_threads: u32,
    base_seed: u32,
    max_path_length: uint,
    min_path_length: uint,
    output_name: String,
    resolution: (), // TODO
    full_report: bool,
}
