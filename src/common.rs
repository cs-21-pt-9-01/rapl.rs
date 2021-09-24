use std::fs;
use std::io::Write;
use std::io;

pub(crate) const UJ_TO_J_FACTOR: f64 = 1000000.;

pub(crate) fn read_power() -> f64 {
    let file_path = "/sys/devices/virtual/powercap/intel-rapl/intel-rapl:0/energy_uj";
    let power = fs::read(file_path).expect("couldnt read file");

    return reading_as_int(&power);
}

pub(crate) fn reading_as_int(reading: &Vec<u8>) -> f64 {
    let power = String::from_utf8_lossy(reading);
    let power_as_int = power.replace("\n", "").parse::<f64>().unwrap();
    return power_as_int;
}

pub(crate) fn spacing(line: String) -> String {
    let col_spacing = 25;
    return " ".repeat(col_spacing - line.len());
}

pub(crate) fn print_headers() {
    print!("time{}J since start{}w since start{}w since last poll\n",
             spacing(format!("time")),
             spacing(format!("J since start")),
             spacing(format!("w since start")));
}

pub(crate) fn print_result_line(time_elapsed: f64, power_j: f64, watts: f64, watts_since_last: f64) {
    print!("{:.0}{}{:.3}{}{:.3}{}{:.3}\r",
               time_elapsed, spacing(format!("{:.0}", time_elapsed)),
               power_j, spacing(format!("{:.3}", power_j)),
               watts, spacing(format!("{:.3}", watts)),
               watts_since_last);
        io::stdout().flush().unwrap();
}