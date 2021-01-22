#[allow(dead_code)]

extern crate reqwest;
extern crate serde;

use serde::{Serialize, Deserialize};
use std::fs;
use std::env;
use std::{thread, time};

#[derive(Deserialize, Debug)]
// Representation of a job reservation response
struct JobSubmitResponse {
    uid: u32,
    user_uid : String,
    user : String,
    walltime : u32,
    queue : String,
    state : String,
    project : String,
    types : Vec<String>,
    mode : String,
    command : String,
    submitted_at : u64,
    started_at : u64,
    message : String,
    properties : String,
    directory : String,
    events : Vec<String>,
    links : Vec<LinkJob>,
    assigned_nodes : Vec<String>
}
#[derive(Deserialize, Debug)]
// Representation of an environment deployment response
struct DeployEnvResponse {
    uid: String,
    site_uid: String,
    user_uid : String,
    environment : String,
    status : String,
    key : String,
    nodes : Vec<String>,
    created_at : u32,
    updated_at : u32,
    links : Vec<LinkJob>
}

#[derive(Deserialize, Debug)]
// Representation of a Link
struct LinkJob {
    rel : String,
    href : String,
    r#type : String,
}

#[derive(Serialize, Debug)]
// Representation of job reservation request
struct ReservationRequest {
    name : String,
    resources : String,
    command : String,
    types : Vec<String>,
}

#[derive(Serialize, Debug)]
// Representation of an environment deployment request
struct DeploymentRequest {
    environment : String,
    nodes : Vec<String>,
    key : String
}

const API_BASE_URL : &str = "https://api.grid5000.fr/3.0/sites/";
const DEPLOY_URL : &str = "/deployments/";
const JOB_URL_PRETTY : &str = "/jobs/?pretty/";
const JOB_URL : &str = "/jobs/";

#[allow(unused_must_use)]
fn main() {
    let args: Vec<String> = env::args().collect();

    let username : &str = &args[1];
    let password : &str = &args[2];
    let site : &str = &args[3];
    let nb_nodes : &str = &args[4];
    let walltime : &str = &args[5];
    let env = "debian10-x64-min";
    let ssh_key_path : &str = &args[6];
    
    let ssh_key : String = get_ssh_key(ssh_key_path).unwrap();

    // Reserve a node and get the resposne from API
    let job_waiting : JobSubmitResponse = reserve_node(username, password, site, nb_nodes, walltime).unwrap();

    // Check if the job's reservation is finished
    let mut job_deployed : JobSubmitResponse = get_reservation(username, password, site, job_waiting.uid.to_string()).unwrap();

    while job_deployed.state != "running" {
        job_deployed = get_reservation(username, password, site, job_waiting.uid.to_string()).unwrap();
    }

    // When job is reserved, deploy environment on node
    deploy_env_on_node(username, password, site, job_deployed.assigned_nodes, env, ssh_key.as_str());

}

// Reserve a node on Grid5000
#[allow(dead_code)]
fn reserve_node(username : &str, password : &str, site : &str, nb_nodes : &str, walltime : &str) -> Result<JobSubmitResponse, reqwest::Error> {

    let api_url = format!("{}{}{}", API_BASE_URL, site, JOB_URL_PRETTY);

    let mut deploy_option : Vec<String> = Vec::new();
    deploy_option.push("deploy".to_string());

    let resource = format!("nodes={},walltime={}", nb_nodes, walltime); 

    let request_body  = ReservationRequest {
        name : "test_magnes.ie".to_string(),
        resources : resource,
        command : "sleep 7200".to_string(),
        types : deploy_option
    };

    let client = reqwest::blocking::Client::new();

    let res = client.post(api_url.as_str())
                                 .json(&request_body)
                                 .basic_auth(username, Some(password))
                                 .send()
                                 .expect("Failed to send request");

    let response_body : JobSubmitResponse = res.json().unwrap();
    println!("{:?}", response_body);
    println!("");

    Ok(response_body)
}

// Check state of reservation with uid = job_uid
#[allow(dead_code)]
fn get_reservation(username : &str, password : &str, site : &str, job_uid : String) -> Result<JobSubmitResponse, reqwest::Error> {

    thread::sleep(time::Duration::from_secs(5));

    let api_url = format!("{}{}{}", API_BASE_URL, site, JOB_URL,);

    let client = reqwest::blocking::Client::new();

    let res = client.get(format!("{}{}", api_url, job_uid).as_str())
                    .basic_auth(username, Some(password))
                    .send()
                    .expect("Failed to send request");

    let response_body : JobSubmitResponse = res.json().unwrap();
    println!("{:?}", response_body);
    println!("");

    Ok(response_body)
}

// Deploy provided environment to specified node
#[allow(dead_code)]
fn deploy_env_on_node(username : &str, password : &str, site : &str, target_nodes : Vec<String>, environment : &str, ssh_key : &str) -> Result<(), reqwest::Error>  {

    let api_url = format!("{}{}{}", API_BASE_URL, site, DEPLOY_URL);

    let request_body  = DeploymentRequest {
        nodes : target_nodes,
        environment : environment.to_string(),
        key : ssh_key.to_string()
    };

    let client = reqwest::blocking::Client::new();

    let res = client.post(api_url.as_str())
                                .json(&request_body)
                                .basic_auth(username, Some(password))
                                .send()
                                .expect("Failed to send request");

    let response_body : DeployEnvResponse = res.json().unwrap();
    println!("{:?}", response_body);
    println!("");

    Ok(())
}

// Delete reservation of node with uid = job_uid
#[allow(dead_code)]
fn delete_job(username : &str, password : &str, site : &str, job_to_delete : String) -> Result<(), reqwest::Error> {

    let api_url = format!("{}{}{}", API_BASE_URL, site, JOB_URL);

    let client = reqwest::blocking::Client::new();
    let res = client.delete(format!("{}{}", api_url, job_to_delete).as_str())
                    .basic_auth(username, Some(password))
                    .send()
                    .expect("Failed to send request");

    // Move and borrow value of `res`
    let response_body = res.text().unwrap();
    println!("{:?}", response_body);
    println!("");
                    
    Ok(())
}

// Get the SSH key from provided file
#[allow(dead_code)]
fn get_ssh_key(file_path : &str) -> Result<String, Box<dyn std::error::Error + 'static>> {
    let ssh_key: String = fs::read_to_string(file_path)?;
    Ok(ssh_key)
}