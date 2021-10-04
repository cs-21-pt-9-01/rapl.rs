use crate::common;
use crate::models;

use std::time::{Duration, Instant};
use std::thread;
use std::path::PathBuf;
use std::process::{Command};
use std::io;
use std::io::Write;

pub(crate) fn live_measurement(poll_delay: u64) {
    let sleep = Duration::from_millis(poll_delay);
    let mut zones = common::setup_rapl_data();
    let mut new_zones: Vec<models::RAPLData> = vec![];

    let start_time = Instant::now();
    let mut prev_time: Instant = start_time;
    #[allow(unused_assignments)]
    let mut now = start_time;

    loop {
        now = Instant::now();
        for zone in zones {
            new_zones.push(common::calculate_power_metrics(
                zone, now, start_time, prev_time
            ));
        }
        zones = new_zones.to_vec();
        new_zones.clear();

        ncurses::clear();
        ncprint!("Press 'q' to quit\n");
        print_result_line!(&zones, true);

        prev_time = now;

        if ncurses::getch() == common::KEY_CODE_EXIT {
            ncurses::endwin();
            break;
        }

        thread::sleep(sleep);
    }
}

pub(crate) fn benchmark(runner: PathBuf, program: PathBuf, args: Vec<String>, n: u64) {
    let zones = common::setup_rapl_data();
    let mut new_zones: Vec<models::RAPLData> = vec![];

    let start_time = Instant::now();

    for i in 0..n {
        if n > 1 {
            println!("Running benchmark iteration {}", i + 1);
        }

        let _out = Command::new(&runner).arg(&program).args(&args).output().expect("Failed to execute command");
    }

    let now = Instant::now();
    for zone in zones {
        new_zones.push(common::calculate_power_metrics(
            zone, now, start_time, start_time
        ));
    }

    print_headers!();
    print_result_line!(&new_zones);
    println!();
}

pub(crate) fn benchmark_interactive(program: PathBuf, poll_delay: u64) {
    let sleep = Duration::from_millis(poll_delay);
    let mut zones = common::setup_rapl_data();
    let mut new_zones: Vec<models::RAPLData> = vec![];

    let start_time = Instant::now();
    let mut prev_time = start_time;
    #[allow(unused_assignments)]
    let mut now = start_time;

    let _out = Command::new(program.to_owned()).spawn().expect("Couldn't execute command");

    loop {
        now = Instant::now();
        for zone in zones {
            new_zones.push(common::calculate_power_metrics(
                zone, now, start_time, prev_time
            ));
        }
        zones = new_zones.to_vec();
        new_zones.clear();

        ncurses::clear();
        ncprint!(format!("Running application {:?}\n", program).as_str());
        ncprint!(format!("'q' or ctrl+c to exit. Ctrl+c will kill {:?} as well\n", program).as_str());
        print_result_line!(&zones, true);

        prev_time = now;

        if ncurses::getch() == common::KEY_CODE_EXIT {
            ncurses::endwin();
            break;
        }

        thread::sleep(sleep);
    }
}

pub(crate) fn inline(metric: String, poll_delay: u64) {
    let choices = vec!["joules", "avg_watt", "avg_watt_curr", "watt_h", "kwatt_h"];
    let file_path = "/sys/devices/virtual/powercap/intel-rapl/intel-rapl:0".to_string();
    match metric.as_str() {
        "joules" => {
            inline_joules(poll_delay, file_path);
        },
        "avg_watt" => {
            inline_avg_watt(poll_delay, file_path);
        },
        "avg_watt_curr" => {
            inline_avg_watt_current(poll_delay, file_path);
        },
        "watt_h" => {
            inline_watt_h(poll_delay, file_path);
        },
        "kwatt_h" => {
            inline_kwatt_h(poll_delay, file_path);
        }
        _ => {
            println!("Couldnt parse input; choices: {:?}", choices);
        }
    }
}

fn inline_joules(poll_delay: u64, file_path: String) {
    let sleep = Duration::from_millis(poll_delay);
    let start_power = common::read_power(file_path.to_owned());

    loop {
        let cur_power = common::read_power(file_path.to_owned());

        print!("\r{:.3}", cur_power - start_power);
        io::stdout().flush().unwrap();

        thread::sleep(sleep);
    }
}

fn inline_avg_watt(poll_delay: u64, file_path: String) {
    let sleep = Duration::from_millis(poll_delay);
    let start_power = common::read_power(file_path.to_owned());
    let start_time = Instant::now();

    loop {
        let cur_power = common::read_power(file_path.to_owned());
        let joules = cur_power - start_power;

        print!("\r{:.3}", joules / start_time.elapsed().as_secs_f64());
        io::stdout().flush().unwrap();

        thread::sleep(sleep);
    }
}

fn inline_avg_watt_current(poll_delay: u64, file_path: String) {
    let sleep = Duration::from_millis(poll_delay);
    let mut prev_power = common::read_power(file_path.to_owned());
    let mut prev_time = Instant::now();

    loop {
        let cur_power = common::read_power(file_path.to_owned());
        let joules = cur_power - prev_power;

        print!("\r{:.3}", joules / prev_time.elapsed().as_secs_f64());
        io::stdout().flush().unwrap();

        prev_time = Instant::now();
        prev_power = cur_power;
        thread::sleep(sleep);
    }
}

fn inline_watt_h(poll_delay: u64, file_path: String) {
    let sleep = Duration::from_millis(poll_delay);
    let start_power = common::read_power(file_path.to_owned());

    loop {
        let cur_power = common::read_power(file_path.to_owned());
        let joules = cur_power - start_power;

        print!("\r{:.5}", common::watt_hours(joules));
        io::stdout().flush().unwrap();

        thread::sleep(sleep);
    }
}

fn inline_kwatt_h(poll_delay: u64, file_path: String) {
    let sleep = Duration::from_millis(poll_delay);
    let start_power = common::read_power(file_path.to_owned());

    loop {
        let cur_power = common::read_power(file_path.to_owned());
        let joules = cur_power - start_power;

        print!("\r{:.5}", common::kwatt_hours(joules));
        io::stdout().flush().unwrap();

        thread::sleep(sleep);
    }
}

pub(crate) fn list(input: String) {
    let choices = vec!["zones"];
    match input.as_str() {
        "zones" => {
            for zone in common::list_rapl() {
                println!("{:?}", zone);
            }
        },
        _ => {
            println!("Malformed input, valid choices: {:?}", choices);
        }
    }
}