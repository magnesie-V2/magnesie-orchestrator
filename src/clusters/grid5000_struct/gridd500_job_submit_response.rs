extern crate serde;

use super::grid5000_link_job::LinkJob;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
// Representation of a job reservation response
pub struct JobSubmitResponse {
    pub uid: u32,
    pub user_uid : String,
    pub user : String,
    pub walltime : u32,
    pub queue : String,
    pub state : String,
    pub project : String,
    pub types : Vec<String>,
    pub mode : String,
    pub command : String,
    pub submitted_at : u64,
    pub started_at : u64,
    pub message : String,
    pub properties : String,
    pub directory : String,
    pub events : Vec<String>,
    pub links : Vec<LinkJob>,
    pub assigned_nodes : Vec<String>
}