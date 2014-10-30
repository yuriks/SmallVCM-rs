#![feature(macro_rules)]
#![feature(slicing_syntax)]
#![allow(dead_code)]

extern crate getopts;

use std::os::set_exit_status;
use config::{Config, LimitTime, LimitIterations};
use std::io::stdio;

mod camera;
mod config;
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

fn render(_config: &Config) -> (f32, int) {
    unimplemented!() // TODO
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
        config.num_threads = 1;
        // TODO config.num_threads = max(1, omp_get_num_procs());
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
        LimitTime(t) => println!("Target:  {} seconds render time", t),
        LimitIterations(n) => println!("Target:  {} iteration(s)", n),
    }

    print!("Running: {}... ", config.algorithm.get_name());
    stdio::flush();
    let (time, _) = render(&config);
    println!("done in {:.2} s", time);

    let extension = config.output_name[].rsplitn(1, '.').next();

    match extension {
        Some("bmp") => config.framebuffer.save_bmp(config.output_name[], 2.2),
        Some("hdr") => config.framebuffer.save_hdr(config.output_name[]),
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
