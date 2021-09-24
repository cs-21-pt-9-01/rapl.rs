mod common;
mod tools;

use structopt::StructOpt;
use std::path::PathBuf;
use std::process;

#[derive(StructOpt)]
#[structopt(
    name = "RAPL.rs",
    author = "PT10xE21",
    about = "RAPL measurement tool",
)]
enum Cli {
    #[structopt(about = "Live measurements")]
    Live {
        /// Delay between polls (ms)
        #[structopt(short = "d", long = "delay", default_value = "1000")]
        delay: u64,
    },
    #[structopt(about = "Measure power consumption of a oneshot script")]
    // TODO: run n times
    Benchmark {
        /// Benchmark runner application, e.g., python
        #[structopt(parse(from_os_str))]
        runner: PathBuf,
        /// Benchmark program
        #[structopt(parse(from_os_str))]
        program: PathBuf,
        /// Args for <program>
        args: Vec<String>
    },
    #[structopt(about = "Measure power consumption of an interactive application")]
    BenchmarkInt {
        /// Benchmark program
        #[structopt(parse(from_os_str))]
        program: PathBuf
    }
}

fn main() {
    match Cli::from_args() {
        Cli::Live { delay } => {
            tools::live_measurement(delay);
        },
        Cli::Benchmark { runner, program, args} => {
            tools::benchmark(runner, program, args);
        },
        Cli::BenchmarkInt { program } => {
            tools::benchmark_interactive(program);
        }
    }
}