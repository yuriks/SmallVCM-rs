#![feature(macro_rules)]
#![feature(trace_macros)]
#![feature(slicing_syntax)]
#![allow(dead_code)]

extern crate getopts;

use std::default::Default;
use std::os::set_exit_status;
use config::Config;

mod config;
mod math;
mod rng;
mod scene;

fn full_report(_a_config: &Config) {
    // TODO
}

fn main() {
    // Setups config based on command line
    let mut config : Config = Default::default();
    // TODO config::ParseCommandline(argc, argv, &mut config);

    if config.num_threads <= 0 {
        config.num_threads = 1;
        // TODO config.num_threads = max(1, omp_get_num_procs());
    }

    if config.full_report {
        full_report(&config);
        set_exit_status(0);
        return;
    }

    if config.scene == None {
        set_exit_status(1);
        return;
    }

    // TODO
}
