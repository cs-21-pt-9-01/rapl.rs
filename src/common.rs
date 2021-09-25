use std::fs;
use std::io::Write;
use std::io;

pub(crate) const UJ_TO_J_FACTOR: f64 = 1000000.;

pub(crate) fn read_power() -> f64 {
    let file_path = "/sys/devices/virtual/powercap/intel-rapl/intel-rapl:0/energy_uj";
    let power = fs::read(file_path).expect("couldnt read file");

    return reading_as_float(&power);
}

pub(crate) fn reading_as_float(reading: &Vec<u8>) -> f64 {
    let power = String::from_utf8_lossy(reading);
    let power_as_float = power.replace("\n", "").parse::<f64>().unwrap();
    return power_as_float;
}

pub(crate) fn spacing(line: String) -> String {
    let col_spacing = 25;
    return " ".repeat(col_spacing - line.len());
}

pub(crate) fn print_headers() {
    print!("time (s){}J since start{}avg w since start{}avg w since last poll{}w/h{}kw/h\n",
             spacing(format!("time (s)")),
             spacing(format!("J since start")),
             spacing(format!("avg w since start")),
             spacing(format!("avg w since last poll")),
             spacing(format!("w/h")));
}

pub(crate) fn print_result_line(time_elapsed: f64, power_j: f64, watts: f64, watts_since_last: f64) {
    let watt_hours = watt_hours(power_j);
    let kwatt_hours = kwatt_hours(power_j);
    print!("\r{:.0}{}{:.3}{}{:.3}{}{:.3}{}{:.5}{}{:.5}",
               time_elapsed, spacing(format!("{:.0}", time_elapsed)),
               power_j, spacing(format!("{:.3}", power_j)),
               watts, spacing(format!("{:.3}", watts)),
               watts_since_last, spacing(format!("{:.3}", watts_since_last)),
               watt_hours, spacing(format!("{:.5}", watt_hours)),
               kwatt_hours);
    io::stdout().flush().unwrap();
}

pub(crate) fn watt_hours(power_j: f64) -> f64 {
    return power_j / 3600.;
}

pub(crate) fn kwatt_hours(power_j: f64) -> f64 {
    return watt_hours(power_j) / 1000.;
}