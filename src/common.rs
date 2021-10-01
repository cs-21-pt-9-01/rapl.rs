use crate::models;

use std::fs;
use std::fs::{DirEntry};
use ncurses;

pub(crate) const UJ_TO_J_FACTOR: f64 = 1000000.;

// ncurses
const COLOUR_BLACK: i16 = 0;
const DEFAULT_COLOUR: i16 = -1;
pub(crate) const HEADER_PAIR: i16 = 1;

pub(crate) fn read_power(file_path: String) -> f64 {
    //let file_path = "/sys/devices/virtual/powercap/intel-rapl/intel-rapl:0/energy_uj";
    let power = fs::read(format!("{}/energy_uj", file_path.to_owned())).expect(format!("Couldn't read file {}/energy_uj", file_path.to_owned()).as_str());

    return reading_as_float(&power) / UJ_TO_J_FACTOR;
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

pub(crate) fn print_headers(ncurses: bool) {
    let headers = vec!["zone", "time(s)", "J", "avg watt", "avg watt curr", "w/h", "kw/h"];
    let mut line: String = "".to_owned();

    for h in headers {
        line.push_str(format!("{}{}", h, spacing(h.to_string())).as_str());
    }

    line = line.trim().to_string();
    line.push_str("\n");
    if ncurses {
        ncurses::attron(ncurses::A_BOLD());
        ncurses::addstr(line.as_str());
        ncurses::attroff(ncurses::A_BOLD());
        ncurses::refresh();
    } else {
        print!("{}", line);
    }
}

#[macro_export]
macro_rules! print_headers {
    ($ncurses: expr) => {
        // what the fuck
        {crate::common::print_headers($ncurses);}
    };
    () => {
        // what the fuck
        {crate::common::print_headers(false);}
    }
}

#[macro_export]
macro_rules! ncprint {
    ($str: expr) => {
        ncurses::addstr($str);
        ncurses::refresh();
    };
}

pub(crate) fn print_result_line(zones: &Vec<models::RAPLData>, ncurses: bool) {
    let mut line: String = "\r".repeat(zones.len()).to_owned();

    for zone in zones {
        let watt_hours = watt_hours(zone.power_j);
        let kwatt_hours = kwatt_hours(zone.power_j);
        let fields = vec![zone.time_elapsed, zone.power_j, zone.watts, zone.watts_since_last, watt_hours, kwatt_hours];
        let zone_name = zone.zone.to_owned();
        line.push_str(format!("{}{}", zone_name.to_owned(), spacing(zone_name.to_owned())).as_str());

        for f in fields {
            line.push_str(format!("{:.5}{}", f, spacing(format!("{:.5}", f))).as_str());
        }

        if zone.zone != zones.last().unwrap().zone {
            line.push_str("\n");
        }
    }

    if ncurses {
        print_headers!(true);
        ncurses::addstr(line.as_str());
        ncurses::refresh();
    } else {
        print!("{}", line);
    }
}

#[macro_export]
macro_rules! print_result_line {
    ($zones: expr, $ncurses: expr) => {
        // what the fuck
        {crate::common::print_result_line($zones, $ncurses);}
    };
    ($zones: expr) => {
        // what the fuck
        {crate::common::print_result_line($zones, false);}
    }
}

pub(crate) fn watt_hours(power_j: f64) -> f64 {
    return power_j / 3600.;
}

pub(crate) fn kwatt_hours(power_j: f64) -> f64 {
    return watt_hours(power_j) / 1000.;
}

pub(crate) fn list_rapl() -> Vec<models::RAPLZone> {
    let base_path = "/sys/devices/virtual/powercap/intel-rapl/";
    let cpus = fs::read_dir(base_path).unwrap();
    let mut zones: Vec<models::RAPLZone> = vec![];

    for cpu in cpus {
        let pkg = parse_rapl_dir(cpu.unwrap());
        match pkg {
            Some(x) => zones.push(x),
            None => continue
        }

        let path = &zones.last().unwrap().path;
        let pkg = fs::read_dir(path).unwrap();

        for core in pkg {
            let core_zone = parse_rapl_dir(core.unwrap());
            match core_zone {
                Some(x) => zones.push(x),
                None => continue
            }
        }
    }

    return zones;
}

fn parse_rapl_dir(item: DirEntry) -> Option<models::RAPLZone> {
    let cleaned_item_name = item.path().display().to_string().split("/").last().unwrap().to_owned();

    if !cleaned_item_name.contains("intel-rapl") {
        return None;
    }

    let item_path = item.path().display().to_string();
    let item_name_data = fs::read(format!("{}/name", item_path))
        .expect(format!("Couldn't read file {}/name", item_path).as_str());
    let item_name = String::from_utf8_lossy(&item_name_data);

    return Some(models::RAPLZone{
        path: item.path().display().to_string(),
        name: item_name.to_string().replace("\n", "")
    });
}

pub(crate) fn setup_rapl_data() -> Vec<models::RAPLData> {
    let sys_zones = list_rapl();
    let mut zones: Vec<models::RAPLData> = vec![];

    for z in sys_zones {
        let data = models::RAPLData{
            path: z.path.to_owned(),
            zone: z.name,
            time_elapsed: 0.,
            power_j: 0.,
            watts: 0.,
            watts_since_last: 0.,
            start_power: read_power(z.path),
            prev_power: 0.
        };
        zones.push(data);
    }

    return zones;
}

pub(crate) fn setup_ncurses() {
    ncurses::initscr();

    if ncurses::has_colors() {
        ncurses::start_color();
        ncurses::init_pair(HEADER_PAIR, COLOUR_BLACK, DEFAULT_COLOUR);
    }
}