
#[derive(Debug)]
pub(crate) struct RAPLZone {
    pub path: String,
    pub name: String
}

#[derive(Debug, Clone)]
pub(crate) struct RAPLData {
    pub path: String,
    pub zone: String,
    pub time_elapsed: f64,
    pub power_j: f64,
    pub watts: f64,
    pub watts_since_last: f64,
    pub start_power: f64,
    pub prev_power: f64
}