use crate::models;

use csv;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::OpenOptions;
use std::path::Path;

pub(crate) fn log_poll_result(system_start_time: SystemTime, tool: String, zone: models::RAPLData) {
    let file_name = format!("{}-{}.csv", tool, system_start_time.duration_since(UNIX_EPOCH)
        .expect("Failed to check duration").as_secs_f64());

    if Path::new(file_name.to_owned().as_str()).exists() {
        let file = OpenOptions::new().write(true).append(true).open(file_name).unwrap();

        let mut wtr = csv::WriterBuilder::default().has_headers(false).from_writer(file);
        wtr.serialize(zone).expect("Failed to write to file");
    } else {
        let file = OpenOptions::new().write(true).create(true).open(file_name).unwrap();

        let mut wtr = csv::Writer::from_writer(file);
        wtr.serialize(zone).expect("Failed to write to file");
    }

}