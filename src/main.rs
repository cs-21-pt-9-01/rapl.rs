use std::time::{Duration, Instant};
use std::fs;
use std::thread;
use std::io::Write;
use std::io;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// Delay between polls (ms)
    #[structopt(short = "d", long = "delay", default_value = "1000")]
    delay: u64
}

fn main() {
    let args = Cli::from_args();
    println!("Poll delay: {}ms", args.delay);

    let sleep = Duration::from_millis(args.delay);

    let uj_to_j = 1000000.;
    let start_time = Instant::now();
    let start_power = read_power();

    let mut prev_power: f64 = 0.;
    let mut prev_time: Instant = start_time;
    let mut watts_since_last = 0.;

    print!("time{}J since start{}w since start{}w since last poll\n",
             spacing(format!("time")),
             spacing(format!("J since start")),
             spacing(format!("w since start")));
    loop {
        let power_uj = read_power();
        let power_since_start = power_uj - start_power;
        let power_j = power_since_start / uj_to_j;
        let now = Instant::now();

        let mut watts = 0.;
        let sample_time = now.duration_since(start_time).as_secs_f64();

        if sample_time > 0. {
            watts = power_j / sample_time;
        }

        if prev_time != start_time {
            watts_since_last = (power_j - prev_power) / now.duration_since(prev_time).as_secs_f64();
        }

        let time_elapsed = start_time.elapsed().as_secs_f64();
        print!("{:.0}{}{:.3}{}{:.3}{}{:.3}\r",
               time_elapsed, spacing(format!("{:.0}", time_elapsed)),
               power_j, spacing(format!("{:.3}", power_j)),
               watts, spacing(format!("{:.3}", watts)),
               watts_since_last);
        io::stdout().flush().unwrap();

        prev_power = power_j;
        prev_time = now;

        thread::sleep(sleep);
    }
}

fn read_power() -> f64 {
    let file_path = "/sys/devices/virtual/powercap/intel-rapl/intel-rapl:0/energy_uj";
    let power = fs::read(file_path).expect("couldnt read file");

    return reading_as_int(&power);
}

fn reading_as_int(reading: &Vec<u8>) -> f64 {
    let power = String::from_utf8_lossy(reading);
    let power_as_int = power.replace("\n", "").parse::<f64>().unwrap();
    return power_as_int;
}

fn spacing(line: String) -> String {
    let col_spacing = 25;
    return " ".repeat(col_spacing - line.len());
}