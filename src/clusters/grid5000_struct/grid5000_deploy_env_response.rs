extern crate serde;

use super::grid5000_link_job::LinkJob;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
/// Representation of an environment deployment response
pub struct DeployEnvResponse {
    pub uid: String,
    pub site_uid: String,
    pub user_uid : String,
    pub environment : String,
    pub status : String,
    pub key : String,
    pub nodes : Vec<String>,
    pub created_at : u32,
    pub updated_at : u32,
    pub links : Vec<LinkJob>
}