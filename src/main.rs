mod common;
mod tools;
mod models;

use structopt::StructOpt;
use std::path::PathBuf;
use ncurses;

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
        program: PathBuf,
        /// Delay between polls (ms)
        #[structopt(short = "d", long = "delay", default_value = "1000")]
        delay: u64
    },
    #[structopt(about = "Inline output of a given metric")]
    Inline {
        /// What to measure
        metric: String,
        /// Delay between polls (ms)
        #[structopt(short = "d", long = "delay", default_value = "1000")]
        delay: u64
    },
    #[structopt(about = "list")]
    List {
        /// What to list
        input: String
    }
}

fn main() {
    match Cli::from_args() {
        Cli::Live { delay } => {
            common::setup_ncurses();
            tools::live_measurement(delay);
        },
        Cli::Benchmark { runner, program, args, n} => {
            tools::benchmark(runner, program, args, n);
        },
        Cli::BenchmarkInt { program, delay } => {
            common::setup_ncurses();
            tools::benchmark_interactive(program, delay);
        },
        Cli::Inline { metric, delay } => {
            tools::inline(metric, delay);
        },
        Cli::List { input } => {
            tools::list(input);
        }
    }
}