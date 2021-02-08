extern crate serde;

use serde::{Serialize};

#[derive(Serialize, Debug)]
// Representation of job reservation request
pub struct ReservationRequest {
    pub name : String,
    pub resources : String,
    pub command : String,
    pub types : Vec<String>,
}