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
    /// Idle data to isolate measurements from - see README.md for details
    #[structopt(short = "i", long = "isolate-from", parse(from_os_str))]
    isolate_file: Option<PathBuf>,
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
        n: u64,
        /// Interval between benchmark runs in seconds
        #[structopt(short = "i", long = "interval", default_value = "0")]
        interval: u64
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
    #[structopt(about = "List utility for various RAPL-related information")]
    List {
        /// What to list
        input: String
    },
    #[structopt(about = "Pretty print last measurement of .csv file")]
    PrettyPrint {
        /// File to print from
        file: PathBuf
    },
    #[structopt(about = "Tools for measuring and generating isolation data")]
    Isolate {
        /// Measure data as a basis for isolation for n minutes - make sure your system is as idle as possible
        #[structopt(short = "m", long = "measure", default_value = "30")]
        measure: u64,
        /// Generate isolation data based on input .csv file
        #[structopt(short = "g", long = "generate")]
        file: Option<PathBuf>
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
        Tool::Benchmark { runner, program, args, n, interval } => {
            tools::do_benchmarks(args_.delay, runner, program, args, n, name, args_.isolate_file, interval);
        },
        Tool::BenchmarkInt { runner, program, background_log } => {
            if !background_log {
                common::setup_ncurses();
            }
            tools::benchmark_interactive(runner, program, args_.delay, system_start_time,
                                         background_log, args_.run_time_limit, name, args_.isolate_file);
        },
        Tool::List { input } => {
            tools::list(input);
        },
        Tool::PrettyPrint { file } => {
            tools::pretty_print(file)
        },
        Tool::Isolate { measure, file } => {
            match file {
                Some(path) => {
                    // generate data
                    tools::generate_isolate_data(path);
                },
                _ => {
                    // measure data basis
                    tools::measure_isolate_data(args_.delay, measure, system_start_time);
                }
            }
        }
    }
}