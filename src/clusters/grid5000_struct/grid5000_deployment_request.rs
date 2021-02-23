extern crate serde;
use serde::{Serialize};

#[derive(Serialize, Debug)]
// Representation of an environment deployment request
pub struct DeploymentRequest {
    pub environment : String,
    pub nodes : Vec<String>,
    pub key : String
}