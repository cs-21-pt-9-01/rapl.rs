use crate::common;
use crate::task;
use crate::models;
use crate::logger;

use csv;
use std::collections::HashMap;
use std::time::{Duration, Instant, SystemTime};
use std::thread;
use std::sync::mpsc;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::io;
use std::io::Write;

pub(crate) fn live_measurement(poll_delay: u64, system_start_time: SystemTime, run_time_limit: Option<u64>, name: String) {
    let tool_name = "live".to_string();
    let sleep = Duration::from_millis(poll_delay);
    let mut zones = common::setup_rapl_data();
    let run_time_limit = run_time_limit.unwrap_or(0);

    let start_time = Instant::now();
    let mut prev_time: Instant = start_time;
    #[allow(unused_assignments)]
    let mut now = start_time;

    loop {
        now = Instant::now();
        zones = common::update_measurements(
            zones.to_owned(), now, start_time, prev_time, system_start_time, tool_name.to_owned(), name.to_owned()
        );

        ncurses::clear();
        ncprint!("Press 'q' to quit\n");
        print_result_line!(&zones, true);

        prev_time = now;

        if ncurses::getch() == common::KEY_CODE_EXIT {
            ncurses::endwin();
            break;
        }

        if common::should_terminate(run_time_limit, now, start_time) {
            common::terminate(&zones);
            break;
        }

        thread::sleep(sleep);
    }
    print_headers!();
    print_result_line!(&zones);
    println!();
}

pub(crate) fn do_benchmarks(poll_delay: u64, runner: Option<PathBuf>, program: PathBuf, args: Vec<String>,
                            n: u64, name: String) {
    for i in 0..n {
        if n > 1 {
            println!("Running benchmark iteration {}", i + 1);
        }

        benchmark(poll_delay, runner.to_owned(), program.to_owned(), args.to_owned(), name.to_owned());
    }
}

pub(crate) fn benchmark(poll_delay: u64, runner: Option<PathBuf>, program: PathBuf, args: Vec<String>,
                        name: String) {
    let tool_name = "benchmark".to_string();
    let zones = common::setup_rapl_data();
    let start_time = Instant::now();
    let iteration_start_time = SystemTime::now();

    let (send, recv) = mpsc::channel();
    let thr = task::spawn_measurement_thread(start_time, iteration_start_time, recv, poll_delay, tool_name.to_owned(), name.to_owned());

    match runner.to_owned() {
        Some(r) => {
            let _out = Command::new(&r).arg(&program).args(&args).output().expect("Failed to execute command");
        },
        None => {
            let _out = Command::new(&program).args(&args).output().expect("Failed to execute command");
        }
    }

    send.send(common::THREAD_KILL).expect("Failed to contact measurement thread");
    thr.join().expect("Failed to wait for measurement thread to finish");

    let now = Instant::now();
    let new_zones = common::update_measurements(
        zones, now, start_time, start_time, iteration_start_time, tool_name.to_owned(), name.to_owned()
    );

    print_headers!();
    print_result_line!(&new_zones);
    println!();
}

pub(crate) fn benchmark_interactive(runner: Option<PathBuf>, program: PathBuf, poll_delay: u64,
                                    system_start_time: SystemTime, background_log: bool,
                                    run_time_limit: Option<u64>, name: String) {
    let tool_name = "benchmark-int".to_string();
    let sleep = Duration::from_millis(poll_delay);
    let mut zones = common::setup_rapl_data();
    let run_time_limit = run_time_limit.unwrap_or(0);

    let start_time = Instant::now();
    let mut prev_time = start_time;
    #[allow(unused_assignments)]
    let mut now = start_time;

    if background_log {
        let (send, recv) = mpsc::channel();
        let thr = task::spawn_measurement_thread(start_time, system_start_time, recv, poll_delay, tool_name.to_owned(), name.to_owned());

        match runner.to_owned() {
            Some(r) => {
                let _out = Command::new(&r).arg(&program).stdout(Stdio::inherit())
                    .stdin(Stdio::inherit()).stderr(Stdio::inherit()).output().expect("Couldn't execute command");
            },
            None => {
                let _out = Command::new(program.to_owned()).stdout(Stdio::inherit())
                    .stdin(Stdio::inherit()).stderr(Stdio::inherit()).output().expect("Couldn't execute command");
            }
        }

        send.send(common::THREAD_KILL).expect("Failed to communicate with measurement thread");
        thr.join().expect("Failed to wait for measurement thread to finish");

        now = Instant::now();
        zones = common::update_measurements(
            zones.to_owned(), now, start_time, prev_time, system_start_time, tool_name.to_owned(), name.to_owned()
        );
    } else {
        match runner.to_owned() {
            Some(r) => {
                let _out = Command::new(&r).arg(&program).spawn().expect("Couldn't execute command");
            },
            None => {
                let _out = Command::new(program.to_owned()).spawn().expect("Couldn't execute command");
            }
        }

        loop {
            now = Instant::now();
            zones = common::update_measurements(
                zones.to_owned(), now, start_time, prev_time, system_start_time, tool_name.to_owned(), name.to_owned()
            );

            ncurses::clear();
            ncprint!(format!("Running application {:?}\n", program).as_str());
            ncprint!(format!("'q' or ctrl+c to exit. Ctrl+c will kill {:?} as well\n", program).as_str());
            print_result_line!(&zones, true);

            prev_time = now;

            if common::should_terminate(run_time_limit, now, start_time) {
                common::terminate(&zones);
                break;
            }

            if ncurses::getch() == common::KEY_CODE_EXIT {
                ncurses::endwin();
                break;
            }

            thread::sleep(sleep);
        }
    }
    print_headers!();
    print_result_line!(&zones);
    println!();
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

pub(crate) fn pretty_print(file: PathBuf) {
    let mut rdr = csv::Reader::from_path(file).unwrap();
    let zones = common::list_rapl();
    let mut out: Vec<models::RAPLData> = vec![];
    for res in rdr.deserialize() {
        let r: models::RAPLData = res.unwrap();
        out.push(r);
    }

    let last = &out[out.len() - zones.len()..].to_vec();

    print_headers!();
    print_result_line!(&last);
    println!();
}

pub(crate) fn measure_isolate_data(poll_delay: u64, minutes: u64, system_start_time: SystemTime) {
    let time_limit_sec = minutes * 60;
    let sleep = Duration::from_millis(poll_delay);
    let mut zones = common::setup_rapl_data();

    let start_time = Instant::now();
    let mut prev_time = start_time;
    #[allow(unused_assignments)]
    let mut now = start_time;

    println!("Measuring isolation data");

    loop {
        now = Instant::now();
        zones = common::update_measurements(
            zones.to_owned(), now, start_time, prev_time, system_start_time, "isolate".to_string(), "idle".to_string()
        );
        prev_time = now;

        print!("\r{} / {} seconds elapsed", now.duration_since(start_time).as_secs(), time_limit_sec);
        io::stdout().flush().unwrap();

        if time_limit_sec > 0 && now.duration_since(start_time).as_secs() >= time_limit_sec {
            break;
        }

        thread::sleep(sleep)
    }

    println!();
    print_headers!();
    print_result_line!(&zones);
    println!();
}

pub(crate) fn generate_isolate_data(csv_file: PathBuf) {
    let mut rdr = csv::Reader::from_path(csv_file).unwrap();
    let zones = common::list_rapl();
    let mut map = HashMap::new();
    let mut out_map = HashMap::new();

    for res in rdr.deserialize() {
        let r: models::RAPLData = res.unwrap();
        map.entry(r.zone.to_owned()).or_insert(vec![]).push(r);
    }

    for zone in zones {
        let zone_data = map.entry(zone.name.to_owned()).or_insert(vec![]);
        zone_data.remove(0);
        let data_len = zone_data.len();
        let mut power_j_step = vec![];
        let mut watts_step = vec![];
        let mut watts_since_last_step = vec![];
        let mut watt_h_step = vec![];
        let mut kwatt_h_step = vec![];

        for n in 0..data_len - 1 {
            if n < data_len {
                power_j_step.push(zone_data[n + 1].power_j - zone_data[n].power_j);
                watt_h_step.push(common::watt_hours(zone_data[n + 1].power_j) -
                    common::watt_hours(zone_data[n].power_j));
                kwatt_h_step.push(common::kwatt_hours(zone_data[n + 1].power_j) -
                    common::kwatt_hours(zone_data[n].power_j));
            }
            watts_step.push(zone_data[n].watts);
            watts_since_last_step.push(zone_data[n].watts_since_last);
        }

        out_map.insert(zone.name, models::IsolateData{
            power_j: models::StatData{
                min: power_j_step.iter().cloned().fold(0./0., f64::min),
                max: power_j_step.iter().cloned().fold(0./0., f64::max),
                avg: power_j_step.iter().sum::<f64>() / power_j_step.len() as f64,
                total: zone_data.last().unwrap().power_j
            },
            watts: models::StatData{
                min: watts_step.iter().cloned().fold(0./0., f64::min),
                max: watts_step.iter().cloned().fold(0./0., f64::max),
                avg: watts_step.iter().sum::<f64>() / watts_step.len() as f64,
                total: 0.
            },
            watts_since_last: models::StatData{
                min: watts_since_last_step.iter().cloned().fold(0./0., f64::min),
                max: watts_since_last_step.iter().cloned().fold(0./0., f64::max),
                avg: watts_since_last_step.iter().sum::<f64>() / watts_since_last_step.len() as f64,
                total: 0.
            },
            watt_h: models::StatData{
                min: watt_h_step.iter().cloned().fold(0./0., f64::min),
                max: watt_h_step.iter().cloned().fold(0./0., f64::max),
                avg: watt_h_step.iter().sum::<f64>() / watt_h_step.len() as f64,
                total: common::watt_hours(zone_data.last().unwrap().power_j)
            },
            kwatt_h: models::StatData{
                min: kwatt_h_step.iter().cloned().fold(0./0., f64::min),
                max: kwatt_h_step.iter().cloned().fold(0./0., f64::max),
                avg: kwatt_h_step.iter().sum::<f64>() / kwatt_h_step.len() as f64,
                total: common::kwatt_hours(zone_data.last().unwrap().power_j)
            }
        });
    }

    logger::log_isolate_data(out_map);
}