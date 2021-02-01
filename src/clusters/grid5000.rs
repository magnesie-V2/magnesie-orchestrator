use chrono::{Utc, Timelike};
use crate::clusters::cluster::Cluster;

pub struct Grid5000;

impl Grid5000 {
    pub fn new(_: &str) -> Grid5000 {
        Grid5000 {}
    }

    pub fn has_green_energy_available(self) -> bool {
        let now = Utc::now();
        let minute = now.minute();
        return if minute % 2 == 0 {
            true
        } else {
            false
        }
    }
}
