#![feature(macro_rules)]
#![feature(slicing_syntax)]
#![allow(dead_code)]

extern crate getopts;
extern crate time;

use config::{Config, RunLimit};
use std::cmp::max;
use std::io::stdio;
use std::os::set_exit_status;
use std::path::Path;

mod camera;
mod config;
mod eyelight;
mod frame;
mod framebuffer;
mod geometry;
mod lights;
mod materials;
mod math;
mod ray;
mod renderer;
mod rng;
mod scene;
mod utils;

fn render(config: &mut Config) -> (f64, u32) {
    use renderer::AbstractRenderer;
    // TODO omp_set_num_threads(config.num_threads);

    let (framebuffer, result) = {
        let mut renderers = Vec::with_capacity(config.num_threads as uint);

        for i in range(0, config.num_threads) {
            let mut renderer = config::create_renderer(config, config.base_seed + i as u32);
            {
                let render_base = renderer.base_mut();
                render_base.max_path_length = config.max_path_length;
                render_base.min_path_length = config.min_path_length;
            }
            renderers.push(renderer);
        }

        let start_time = time::precise_time_s();
        let mut iter = 0;

        match config.run_limit {
            RunLimit::Time(max_time) => {
                // TODO #pragma omp parallel
                while time::precise_time_s() < start_time + max_time {
                    let thread_id = 0; // TODO omp_get_thread_num();
                    renderers[thread_id].run_iteration(iter);
                    // TODO #pragma omp atomic
                    iter += 1;
                }
            },
            RunLimit::Iterations(iterations) => {
                // TODO #pragma omp parallel for
                for iter in range(0, iterations) {
                    let thread_id = 0; // TODO omp_get_thread_num();
                    renderers[thread_id].run_iteration(iter);
                }
                iter = iterations;
            },
        }

        let end_time = time::precise_time_s();

        let mut used_renderers = 0u;

        let mut framebuffer = None;

        for renderer in renderers.iter_mut() {
            if !renderer.base().was_used() {
                continue;
            }

            let renderer_fb = renderer.base().get_framebuffer();

            match framebuffer {
                None => framebuffer = Some(renderer_fb),
                Some(ref mut framebuffer) => framebuffer.add(&renderer_fb),
            };

            used_renderers += 1;
        }

        match framebuffer {
            Some(ref mut framebuffer) => framebuffer.scale(1.0 / used_renderers as f32),
            None => unreachable!(),
        }
        (framebuffer, (end_time - start_time, iter + 1))
    };

    config.framebuffer = framebuffer;

    result
}

fn full_report(_config: &Config) {
    unimplemented!() // TODO
}

fn main() {
    // Setups config based on command line
    let mut config = match config::parse_commandline(std::os::args()[]) {
        Ok(config) => config,
        Err(ref s) if s[] == "" => return,
        Err(ref s) => {
            stdio::println(s[]);
            set_exit_status(1);
            return;
        },
    };

    if config.num_threads <= 0 {
        config.num_threads = max(1, std::os::num_cpus());
    }

    if config.full_report {
        full_report(&config);
        set_exit_status(0);
        return;
    }

    if config.scene.is_none() {
        set_exit_status(1);
        return;
    }

    println!("Scene:   {}\n", config.scene.as_ref().unwrap().scene_name);
    match config.run_limit {
        RunLimit::Time(t) => println!("Target:  {} seconds render time", t),
        RunLimit::Iterations(n) => println!("Target:  {} iteration(s)", n),
    }

    print!("Running: {}... ", config.algorithm.get_name());
    stdio::flush();
    let (time, _) = render(&mut config);
    println!("done in {:.2} s", time);

    let extension = config.output_name[].rsplitn(1, '.').next();
    let path = Path::new(config.output_name[]);

    match extension {
        Some("bmp") => config.framebuffer.unwrap().save_bmp(&path, 2.2).unwrap(),
        Some("hdr") => config.framebuffer.unwrap().save_hdr(&path).unwrap(),
        Some(other_ext) => {
            println!("Used unknown extension {}", other_ext);
            set_exit_status(1);
        }
        None => {
            println!("Output filename has no extension!");
            set_exit_status(1);
        }
    }
}
