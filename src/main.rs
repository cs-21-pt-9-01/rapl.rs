mod common;
mod tools;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// Delay between polls (ms)
    #[structopt(short = "d", long = "delay", default_value = "1000")]
    delay: u64,
    /// Perform live measurements (default)
    #[structopt(short = "l", long = "live")]
    live: bool
}

fn main() {
    let args = Cli::from_args();
    println!("Poll delay: {}ms", args.delay);

    if args.live {
        tools::live_measurement(args.delay);
    }
}