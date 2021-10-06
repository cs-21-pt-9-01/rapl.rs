use crate::common;
use crate::models;
use crate::logger;

use std::thread;
use std::time::{Instant, SystemTime};
use std::thread::JoinHandle;
use std::sync::mpsc::Receiver;
use std::borrow::Borrow;

// TODO: static dir for logs
// TODO: chown log files to user
// TODO: add sample log
pub(crate) fn spawn_measurement_thread(start_time: Instant, system_start_time: SystemTime,
                                       recv: Receiver<i8>, poll_delay: u64, tool_name: String) -> JoinHandle<()> {
    let thr = thread::spawn(move || {
        let mut tzones = common::setup_rapl_data().to_owned();
        let mut thread_zones: Vec<models::RAPLData> = vec![];
        let mut prev_time = start_time.to_owned();
        // reassign locally - unsafe otherwise
        let trecv = recv;
        let mut run = true;
        #[allow(unused_assignments)]
        let mut now = Instant::now();

        while run {
            now = Instant::now();
            tzones = common::update_measurements(
                tzones.to_owned(), now, start_time, prev_time, system_start_time, tool_name.to_owned()
            );
            thread_zones.clear();
            prev_time = now;

            let sleep_from = Instant::now();
            while sleep_from.elapsed().as_millis() < poll_delay as u128 {
                match trecv.try_recv() {
                    Ok(msg) => {
                        if msg == common::THREAD_KILL {
                            let now = Instant::now();
                            let _ = common::update_measurements(
                                tzones.to_owned(), now, start_time, prev_time, system_start_time, tool_name.to_owned()
                            );
                            run = false;
                            break;
                        }
                    },
                    Err(_) => {}
                }
            }
        }
    });

    return thr;
}