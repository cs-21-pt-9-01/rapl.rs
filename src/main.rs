#[macro_use] mod common;
mod tools;
mod models;
mod logger;
mod task;

use structopt::StructOpt;
use std::path::PathBuf;
use std::time::SystemTime;

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
    /// Terminate after time limit (s)
    #[structopt(short = "t", long = "terminate-after")]
    run_time_limit: Option<u64>,
    /// Benchmark name - to easily discern csv output
    #[structopt(short = "n", long = "name")]
    name: Option<String>,
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
        /// Benchmark requires <runner> to execute
        #[structopt(short = "r", long = "runner", parse(from_os_str))]
        runner: Option<PathBuf>,
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
        /// Benchmark requires <runner> to execute
        #[structopt(short = "r", long = "runner", parse(from_os_str))]
        runner: Option<PathBuf>,
        /// Benchmark program
        #[structopt(parse(from_os_str))]
        program: PathBuf,
        /// Log in background and post a summary on exit
        #[structopt(short = "b", long = "bg-log")]
        background_log: bool,
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
    },
    #[structopt(about = "Pretty print last measurement of .csv file")]
    PrettyPrint {
        /// File to print from
        file: PathBuf
    }
}

fn main() {
    let system_start_time = SystemTime::now();
    let args_ = Cli::from_args();
    let name = args_.name.unwrap_or(String::from(""));
    match args_.tool {
        Tool::Live { } => {
            common::setup_ncurses();
            tools::live_measurement(args_.delay, system_start_time, args_.run_time_limit, name);
        },
        Tool::Benchmark { runner, program, args, n } => {
            tools::benchmark(args_.delay, runner, program, args, n, system_start_time, name);
        },
        Tool::BenchmarkInt { runner, program, background_log } => {
            if !background_log {
                common::setup_ncurses();
            }
            tools::benchmark_interactive(runner, program, args_.delay, system_start_time, background_log, args_.run_time_limit, name);
        },
        Tool::Inline { metric} => {
            tools::inline(metric, args_.delay);
        },
        Tool::List { input } => {
            tools::list(input);
        },
        Tool::PrettyPrint { file } => {
            tools::pretty_print(file)
        }
    }
}