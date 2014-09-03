#![feature(macro_rules)]
#![feature(trace_macros)]
#![allow(dead_code)]

use std::default::Default;
use config::Config;

mod config;
mod math;
mod rng;

fn main() {
    // Setups config based on command line
    let mut config : Config = Default::default();
    // TODO config::ParseCommandline(argc, argv, &mut config);

    if config.num_threads <= 0 {
        config.num_threads = 1;
        // TODO config.num_threads = max(1, omp_get_num_procs());
    }

    // TODO
}
