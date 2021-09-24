use std::fs;

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