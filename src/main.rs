mod common;
mod tools;

use structopt::StructOpt;
use std::path::PathBuf;
use std::process;

#[derive(StructOpt)]
enum Cli {
    Live {
        /// Delay between polls (ms)
        #[structopt(short = "d", long = "delay", default_value = "1000")]
        delay: u64,
    },
    Benchmark {
        /// Benchmark runner application, e.g., python
        #[structopt(parse(from_os_str))]
        runner: PathBuf,
        /// Benchmark program
        #[structopt(parse(from_os_str))]
        program: PathBuf,
        /// Args for <program>
        args: Vec<String>
    }
    // TODO: benchmark interactive - e.g., cura
}

fn main() {
    match Cli::from_args() {
        Cli::Live { delay } => {
            tools::live_measurement(delay);
        },
        Cli::Benchmark { runner, program, args} => {
            tools::benchmark(runner, program, args);
        },
    }
}