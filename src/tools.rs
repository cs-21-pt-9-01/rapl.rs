use crate::common;
use std::time::{Duration, Instant};
use std::{io, thread};
use std::io::Write;

pub(crate) fn live_measurement(poll_delay: u64) {
    let sleep = Duration::from_millis(poll_delay);

    let uj_to_j = 1000000.;
    let start_time = Instant::now();
    let start_power = common::read_power();

    let mut prev_power: f64 = 0.;
    let mut prev_time: Instant = start_time;
    let mut watts_since_last = 0.;

    print!("time{}J since start{}w since start{}w since last poll\n",
             common::spacing(format!("time")),
             common::spacing(format!("J since start")),
             common::spacing(format!("w since start")));
    loop {
        let power_uj = common::read_power();
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
               time_elapsed, common::spacing(format!("{:.0}", time_elapsed)),
               power_j, common::spacing(format!("{:.3}", power_j)),
               watts, common::spacing(format!("{:.3}", watts)),
               watts_since_last);
        io::stdout().flush().unwrap();

        prev_power = power_j;
        prev_time = now;

        thread::sleep(sleep);
    }
}