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
    Benchmark {
        /// Benchmark runner application, e.g., python
        #[structopt(parse(from_os_str))]
        runner: PathBuf,
        /// Benchmark program
        #[structopt(parse(from_os_str))]
        program: PathBuf,
        /// Args for <program>
        args: Vec<String>,
        /// Amount of times to run benchmark
        #[structopt(short = "n", default_value = "1")]
        n: u64
    },
    #[structopt(about = "Measure power consumption of an interactive application")]
    BenchmarkInt {
        /// Benchmark program
        #[structopt(parse(from_os_str))]
        program: PathBuf
    },
    Inline {
        /// What to measure
        metric: String,
        /// Delay between polls (ms)
        #[structopt(short = "d", long = "delay", default_value = "1000")]
        delay: u64
    }
}

fn main() {
    match Cli::from_args() {
        Cli::Live { delay } => {
            tools::live_measurement(delay);
        },
        Cli::Benchmark { runner, program, args, n} => {
            tools::benchmark(runner, program, args, n);
        },
        Cli::BenchmarkInt { program } => {
            tools::benchmark_interactive(program);
        },
        Cli::Inline { metric, delay } => {
            tools::inline(metric, delay);
        }
    }
}