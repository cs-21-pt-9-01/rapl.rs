use crate::common;

use std::time::{Duration, Instant};
use std::thread;
use std::path::PathBuf;
use std::process::Command;
use std::io;
use std::io::Write;

pub(crate) fn live_measurement(poll_delay: u64) {
    let sleep = Duration::from_millis(poll_delay);

    let start_time = Instant::now();
    let start_power = common::read_power();

    let mut prev_power: f64 = 0.;
    let mut prev_time: Instant = start_time;
    let mut watts_since_last = 0.;

    common::print_headers();

    loop {
        let power_uj = common::read_power();
        let power_since_start = power_uj - start_power;
        let power_j = power_since_start / common::UJ_TO_J_FACTOR;
        let now = Instant::now();

        let mut watts = 0.;
        let sample_time = now.duration_since(start_time).as_secs_f64();

        if sample_time > 0. {
            watts = power_j / sample_time;
        }

        if prev_time != start_time {
            watts_since_last = (power_j - prev_power) / now.duration_since(prev_time).as_secs_f64();
        }

        common::print_result_line(
            start_time.elapsed().as_secs_f64(),
            power_j,
            watts,
            watts_since_last
        );

        prev_power = power_j;
        prev_time = now;

        thread::sleep(sleep);
    }
}

pub(crate) fn benchmark(runner: PathBuf, program: PathBuf, args: Vec<String>, n: u64) {
    let start_time = Instant::now();
    let start_power = common::read_power();

    for i in 0..n {
        if n > 1 {
            println!("Running benchmark iteration {}", i + 1);
        }

        let _out = Command::new(&runner).arg(&program).args(&args).output().expect("Failed to execute command");
    }

    let end_power = common::read_power();

    let power_j = (end_power - start_power) / common::UJ_TO_J_FACTOR;
    let watts = power_j / start_time.elapsed().as_secs_f64();

    common::print_headers();
    common::print_result_line(start_time.elapsed().as_secs_f64(), power_j, watts, 0.);
    println!();
}

pub(crate) fn benchmark_interactive(program: PathBuf) {
    let start_time = Instant::now();
    let start_power = common::read_power();

    println!("Running application {:?}. Exit the application to stop; Ctrl+C will discard results", program);
    let _out = Command::new(program).output().expect("Failed to execute command");

    let end_power = common::read_power();

    let power_j = (end_power - start_power) / common::UJ_TO_J_FACTOR;
    let watts = power_j / start_time.elapsed().as_secs_f64();

    common::print_headers();
    common::print_result_line(start_time.elapsed().as_secs_f64(), power_j, watts, 0.);
    println!();
}

pub(crate) fn inline(metric: String, poll_delay: u64) {
    let choices = vec!["joules", "avg_watt", "avg_watt_curr", "watt_h", "kwatt_h"];
    match metric.as_str() {
        "joules" => {
            inline_joules(poll_delay);
        },
        "avg_watt" => {
            inline_avg_watt(poll_delay);
        },
        "avg_watt_curr" => {
            inline_avg_watt_current(poll_delay);
        },
        "watt_h" => {
            inline_watt_h(poll_delay);
        },
        "kwatt_h" => {
            inline_kwatt_h(poll_delay);
        }
        _ => {
            println!("Couldnt parse input; choices: {:?}", choices);
        }
    }
}

fn inline_joules(poll_delay: u64) {
    let sleep = Duration::from_millis(poll_delay);
    let start_power = common::read_power();

    loop {
        let cur_power = common::read_power();

        print!("\r{:.3}", (cur_power - start_power) / common::UJ_TO_J_FACTOR);
        io::stdout().flush().unwrap();

        thread::sleep(sleep);
    }
}

fn inline_avg_watt(poll_delay: u64) {
    let sleep = Duration::from_millis(poll_delay);
    let start_power = common::read_power();
    let start_time = Instant::now();

    loop {
        let cur_power = common::read_power();
        let joules = (cur_power - start_power) / common::UJ_TO_J_FACTOR;
        print!("\r{:.3}", joules / start_time.elapsed().as_secs_f64());
        io::stdout().flush().unwrap();

        thread::sleep(sleep);
    }
}

fn inline_avg_watt_current(poll_delay: u64) {
    let sleep = Duration::from_millis(poll_delay);
    let mut prev_power = common::read_power();
    let mut prev_time = Instant::now();

    loop {
        let cur_power = common::read_power();
        let joules = (cur_power - prev_power) / common::UJ_TO_J_FACTOR;
        print!("\r{:.3}", joules / prev_time.elapsed().as_secs_f64());
        io::stdout().flush().unwrap();

        prev_time = Instant::now();
        prev_power = cur_power;
        thread::sleep(sleep);
    }
}

fn inline_watt_h(poll_delay: u64) {
    let sleep = Duration::from_millis(poll_delay);
    let start_power = common::read_power();

    loop {
        let cur_power = common::read_power();
        let joules = (cur_power - start_power) / common::UJ_TO_J_FACTOR;

        print!("\r{:.5}", common::watt_hours(joules));
        io::stdout().flush().unwrap();

        thread::sleep(sleep);
    }
}

fn inline_kwatt_h(poll_delay: u64) {
    let sleep = Duration::from_millis(poll_delay);
    let start_power = common::read_power();

    loop {
        let cur_power = common::read_power();
        let joules = (cur_power - start_power) / common::UJ_TO_J_FACTOR;

        print!("\r{:.5}", common::kwatt_hours(joules));
        io::stdout().flush().unwrap();

        thread::sleep(sleep);
    }
}