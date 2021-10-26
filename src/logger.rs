use crate::models;
use crate::common;

use csv;
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::os::unix::fs::PermissionsExt;

pub(crate) fn log_poll_result(system_start_time: SystemTime, tool: String, zone: models::RAPLData,
                              benchmark_name: String) {
    let file_name = common::create_log_file_name(benchmark_name, tool, system_start_time);

    if Path::new(file_name.to_owned().as_str()).exists() {
        let file = OpenOptions::new().write(true).append(true).open(file_name.to_owned()).unwrap();

        let mut wtr = csv::WriterBuilder::default().has_headers(false).from_writer(file);
        wtr.serialize(zone).expect("Failed to write to file");

        let mut perms = fs::metadata(file_name.to_owned()).unwrap().permissions();
        perms.set_mode(0o666);
        fs::set_permissions(file_name.to_string(), perms).expect("Failed to set permissions for file");
    } else {
        let file = OpenOptions::new().write(true).create(true).open(file_name).unwrap();

        let mut wtr = csv::Writer::from_writer(file);
        wtr.serialize(zone).expect("Failed to write to file");
    }

}

pub(crate) fn log_isolate_data(map: HashMap<String, models::IsolateData>) {
    let file_name = format!("isolate-data-{}.json", SystemTime::now()
        .duration_since(UNIX_EPOCH).expect("Failed to check duration").as_secs_f64());
    let mut file = OpenOptions::new().write(true).create(true).open(file_name).unwrap();
    let json = serde_json::to_string_pretty(&map).unwrap();

    file.write(json.as_bytes()).expect("Failed to write isolation data to file");
    drop(file);
}