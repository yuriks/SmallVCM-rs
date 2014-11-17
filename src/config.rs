use math::Vec2i;
use math::vec2;
use std::default::Default;
use scene;
use scene::{BoxMask, Scene};
use renderer::AbstractRenderer;
use framebuffer::Framebuffer;

enum Algorithm {
    EyeLight,
    PathTracing,
    LightTracing,
    ProgressivePhotonMapping,
    BidirectionalPhotonMapping,
    BidirectionalPathTracing,
    VertexConnectionMerging,
}

impl Algorithm {
    pub fn get_name(self) -> &'static str {
        match self {
            EyeLight => "eye light",
            PathTracing => "path tracing",
            LightTracing => "light tracing",
            ProgressivePhotonMapping => "progressive photon mapping",
            BidirectionalPhotonMapping => "bidirectional photon mapping",
            BidirectionalPathTracing => "bidirectional path tracing",
            VertexConnectionMerging => "vertex connection and merging",
        }
    }

    fn get_acronym(self) -> &'static str {
        match self {
            EyeLight => "el",
            PathTracing => "pt",
            LightTracing => "lt",
            ProgressivePhotonMapping => "ppm",
            BidirectionalPhotonMapping => "bpm",
            BidirectionalPathTracing => "bpt",
            VertexConnectionMerging => "vcm",
        }
    }

    fn from_acronym(s: &str) -> Option<Algorithm> {
        Some(match s {
            "el" => EyeLight,
            "pt" => PathTracing,
            "lt" => LightTracing,
            "ppm" => ProgressivePhotonMapping,
            "bpm" => BidirectionalPhotonMapping,
            "bpt" => BidirectionalPathTracing,
            "vcm" => VertexConnectionMerging,
            _ => return None,
        })
    }
}

pub enum RunLimit {
    LimitIterations(u32),
    LimitTime(f64),
}

pub struct Config {
    pub scene: Option<Scene>,
    pub algorithm: Algorithm,
    pub run_limit: RunLimit,
    radius_factor: f32,
    radius_alpha: f32,
    pub framebuffer: Option<Framebuffer>,
    pub num_threads: uint,
    pub base_seed: u32,
    pub max_path_length: u32,
    pub min_path_length: u32,
    pub output_name: String,
    resolution: Vec2i,
    pub full_report: bool,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            scene: None,
            algorithm: VertexConnectionMerging,
            run_limit: LimitIterations(1),
            radius_factor: 0.003,
            radius_alpha: 0.75,
            framebuffer: None,
            num_threads: 0,
            base_seed: 1234,
            max_path_length: 10,
            min_path_length: 0,
            output_name: "".to_string(),
            // Explicit literal type is a workaround for rust#18954
            resolution: vec2(512i32, 512),
            full_report: false,
        }
    }
}

pub fn create_renderer<'a, 'b>(config: &'a Config, seed: u32) -> Box<AbstractRenderer<'a> + 'a> {
    let scene = match config.scene { Some(ref x) => x, None => unreachable!() };

    match config.algorithm {
        EyeLight => box ::eyelight::EyeLight::new(scene, seed),
        // TODO
        _ => unimplemented!()
    }
}

fn get_scene_config(scene_id: uint) -> Option<BoxMask> {
    match scene_id {
        0 => Some(scene::GLOSSY_FLOOR | scene::BOTH_SMALL_SPHERES  | scene::LIGHT_SUN),
        1 => Some(scene::GLOSSY_FLOOR | scene::LARGE_MIRROR_SPHERE | scene::LIGHT_CEILING),
        2 => Some(scene::GLOSSY_FLOOR | scene::BOTH_SMALL_SPHERES  | scene::LIGHT_POINT),
        3 => Some(scene::GLOSSY_FLOOR | scene::BOTH_SMALL_SPHERES  | scene::LIGHT_BACKGROUND),
        _ => None
    }
}

fn default_filename(scene_config: BoxMask, scene: &Scene, algorithm: Algorithm) -> String {
    let mut filename = String::new();

    if scene_config.contains(scene::GLOSSY_FLOOR) {
        filename.push_str("g");
    }

    filename.push_str(scene.scene_acronym[]);

    filename.push_str("_");
    filename.push_str(algorithm.get_acronym());

    filename.push_str(".bmp");

    filename
}

fn print_help(_argv: &[String]) {
    // TODO
}

pub fn parse_commandline(argv: &[String]) -> Result<Config, String> {
    use getopts::{getopts, optflag, optopt};

    let mut config : Config = Default::default();

    let opts = [
        optflag("h", "help", "Displays usage information."),
        optflag("", "report", "Renders all scenes using all algorithms and generates an index.html file."),
        optopt("s", "", "Selects the scene.", "sceneID"),
        optopt("a", "", "Selects the rendering algorithm.", "algorithm"),
        optopt("i", "", "Number of iterations to run the algorithm for.", "iterations"),
        optopt("t", "", "Number of seconds to run the algorithm for.", "seconds"),
        optopt("o", "", "User specified output name, with extension .bmp or .hdr.", "output_name"),
    ];
    let matches = getopts(argv, opts).unwrap();

    if matches.opt_present("h") {
        print_help(argv);
        return Err("".to_string());
    }

    let scene_config = match matches.opt_str("s") {
        Some(scene_num_str) =>
            match from_str::<uint>(scene_num_str[])
                    .and_then(|id| get_scene_config(id)) {
                Some(scene_config) => scene_config,
                _ => return Err(
                    format!("Invalid scene id \"{}\", please see help (-h).", scene_num_str)),
            },
        None => get_scene_config(0).unwrap(),
    };

    match matches.opt_str("a") {
        Some(algorithm_str) => match Algorithm::from_acronym(algorithm_str[]) {
            Some(algorithm) => config.algorithm = algorithm,
            _ => return Err(
                format!("Invalid algorithm \"{}\", please see help (-h).", algorithm_str)),
        },
        None => (),
    }

    match matches.opt_str("i") {
        Some(iterations_str) => match from_str::<u32>(iterations_str[]) {
            Some(iterations) if iterations >= 1 => config.run_limit = LimitIterations(iterations),
            _ => return Err(format!(
                "Invalid iteration count \"{}\", please see help (-h).", iterations_str)),
        },
        None => (),
    }

    match matches.opt_str("t") {
        Some(time_str) => match from_str::<f64>(time_str[]) {
            Some(time) if time >= 0.0 => config.run_limit = LimitTime(time),
            _ => return Err(format!(
                "Invalid time \"{}\", please see help (-h).", time_str)),
        },
        None => (),
    }

    if matches.opt_present("report") {
        config.full_report = true;
        // In report mode, the scene and algorithm options are ignored and managed by the reporter.
        return Ok(config);
    }

    let mut scene = Scene::load_cornell_box(config.resolution, scene_config);
    scene.build_scene_sphere();
    config.scene = Some(scene);

    config.output_name = match matches.opt_str("o") {
        Some(output_name) => if output_name.len() > 0 {
            output_name
        } else {
            return Err(format!(
                    "Invalid output name \"{}\", please see help (-h).", output_name));
        },
        // Generate a default output name if none was specified
        None => default_filename(scene_config, config.scene.as_ref().unwrap(), config.algorithm),
    };

    // Add a default extension if none's present
    if !config.output_name[].ends_with(".bmp") &&
       !config.output_name[].ends_with(".hdr")
    {
        config.output_name.push_str(".bmp");
    }

    Ok(config)
}
