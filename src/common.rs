use crate::models;
use crate::logger;

use std::fs;
use std::fs::{DirEntry};
use std::time::{Instant, SystemTime};

use ncurses;

pub(crate) const UJ_TO_J_FACTOR: f64 = 1000000.;

// ncurses
const COLOUR_BLACK: i16 = 0;
const DEFAULT_COLOUR: i16 = -1;
pub(crate) const HEADER_PAIR: i16 = 1;
pub(crate) const KEY_CODE_EXIT: i32 = 113;  // q

// threads
pub(crate) const THREAD_KILL: i8 = 1;

pub(crate) fn read_power(file_path: String) -> f64 {
    let power = fs::read(format!("{}/energy_uj", file_path.to_owned())).expect(format!("Couldn't read file {}/energy_uj", file_path.to_owned()).as_str());

    return reading_as_float(&power) / UJ_TO_J_FACTOR;
}

pub(crate) fn read_power_limit(file_path: String) -> f64 {
    let limit = fs::read(format!("{}/max_energy_range_uj", file_path.to_owned())).expect(format!("Couldn't read file {}/max_energy_range_uj", file_path.to_owned()).as_str());

    return reading_as_float(&limit) / UJ_TO_J_FACTOR;
}

pub(crate) fn reading_as_float(reading: &Vec<u8>) -> f64 {
    let power = String::from_utf8_lossy(reading);
    let power_as_float = power.replace("\n", "").parse::<f64>().unwrap();
    return power_as_float;
}

pub(crate) fn spacing(line: String) -> String {
    // 25 and 30 makes for fucky formatting:
    // w/h is misaligned; line break on small monitors, respectively
    let col_spacing = 28;
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

        line = line.trim().to_string();

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
        let start_power = read_power(z.path.to_owned());
        let data = models::RAPLData{
            path: z.path,
            zone: z.name,
            time_elapsed: 0.,
            power_j: 0.,
            watts: 0.,
            watts_since_last: 0.,
            start_power,
            prev_power: 0.,
            prev_power_reading: start_power
        };
        zones.push(data);
    }

    return zones;
}

pub(crate) fn setup_ncurses() {
    let w = ncurses::initscr();
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    ncurses::nodelay(w, true);

    if ncurses::has_colors() {
        ncurses::start_color();
        ncurses::init_pair(HEADER_PAIR, COLOUR_BLACK, DEFAULT_COLOUR);
    }
}

pub(crate) fn kill_ncurses() {
    ncurses::endwin();
    ncurses::reset_shell_mode();
}

pub(crate) fn calculate_power_metrics(zone: models::RAPLData, now: Instant,
                                      start_time: Instant, prev_time: Instant) -> models::RAPLData {
    let cur_power_j = read_power(zone.path.to_owned());

    #[allow(unused_assignments)]
    let mut power_j = 0.;
    let mut watts = 0.;
    let mut watts_since_last = 0.;

    // if RAPL overflow has occurred
    // or if we have done a full RAPL cycle
    if zone.start_power >= cur_power_j || zone.power_j >= cur_power_j {
        // if our previous reading was pre-overflow, we simply add the new reading
        // otherwise we add the difference
        if zone.prev_power_reading > cur_power_j {
            let power_limit = read_power_limit(zone.path.to_owned());
            power_j = (power_limit - zone.prev_power_reading) + cur_power_j + zone.power_j;
        } else {
            power_j = (cur_power_j - zone.prev_power_reading) + zone.power_j;
        }
    } else {
        power_j = cur_power_j - zone.start_power;
    }

    let sample_time = now.duration_since(start_time).as_secs_f64();
    if sample_time > 0. {
        watts = power_j / sample_time;
    }

    if prev_time > start_time {
        watts_since_last = (power_j - zone.power_j) / now.duration_since(prev_time).as_secs_f64();
    }

    return models::RAPLData{
        path: zone.path,
        zone: zone.zone,
        time_elapsed: start_time.elapsed().as_secs_f64(),
        power_j,
        watts,
        watts_since_last,
        start_power: zone.start_power,
        prev_power: zone.power_j,
        prev_power_reading: cur_power_j
    }
}

pub(crate) fn update_measurements(zones: Vec<models::RAPLData>, now: Instant, start_time: Instant,
                                  prev_time: Instant, system_start_time: SystemTime, tool_name: String) -> Vec<models::RAPLData> {
    let mut res: Vec<models::RAPLData> = vec![];

    for zone in zones {
        let new_zone = calculate_power_metrics(zone, now, start_time, prev_time);
        logger::log_poll_result(system_start_time, tool_name.to_owned(), new_zone.to_owned());
        res.push(new_zone);
    }

    return res.to_vec();
}