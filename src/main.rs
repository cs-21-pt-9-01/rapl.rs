#[macro_use] mod common;
mod tools;
mod models;

use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt)]
#[structopt(
    name = "RAPL.rs",
    author = "cs-21-pt-9-01",
    about = "RAPL measurement tool",
)]
struct Cli {
    /// Delay between polls (ms)
    #[structopt(short = "d", long = "delay", default_value = "1000")]
    delay: u64,
    /// Tool to use
    #[structopt(subcommand)]
    tool: Tool
}

#[derive(StructOpt)]
enum Tool {
    #[structopt(about = "Live measurements")]
    Live {},
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
    },
    #[structopt(about = "Inline output of a given metric")]
    Inline {
        /// What to measure
        metric: String,
    },
    #[structopt(about = "List utility for various RAPL-related information")]
    List {
        /// What to list
        input: String
    }
}

fn main() {
    let args = Cli::from_args();
    match args.tool {
        Tool::Live { } => {
            common::setup_ncurses();
            tools::live_measurement(args.delay);
        },
        Tool::Benchmark { runner, program, args, n} => {
            tools::benchmark(runner, program, args, n);
        },
        Tool::BenchmarkInt { program} => {
            common::setup_ncurses();
            tools::benchmark_interactive(program, args.delay);
        },
        Tool::Inline { metric} => {
            tools::inline(metric, args.delay);
        },
        Tool::List { input } => {
            tools::list(input);
        }
    }
}