use crate::common;
use crate::task;
use crate::models;

use csv;
use std::time::{Duration, Instant, SystemTime};
use std::thread;
use std::sync::mpsc;
use std::path::PathBuf;
use std::process::{Command, Stdio};

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