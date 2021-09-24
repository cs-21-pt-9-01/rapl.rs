use crate::common;
use std::time::{Duration, Instant};
use std::thread;

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