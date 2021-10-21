use serde;
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub(crate) struct RAPLZone {
    pub path: String,
    pub name: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RAPLData {
    #[serde(skip_serializing, skip_deserializing)]
    pub path: String,
    pub zone: String,
    pub time_elapsed: f64,
    pub power_j: f64,
    pub watts: f64,
    pub watts_since_last: f64,
    pub start_power: f64,
    pub prev_power: f64,
    pub prev_power_reading: f64
}