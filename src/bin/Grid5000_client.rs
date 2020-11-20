extern crate reqwest;
extern crate serde;

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::env;

#[derive(Deserialize, Debug)]
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
struct LinkJob {
    rel : String,
    href : String,
    r#type : String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let username : &str = &args[1];
    let password : &str = &args[2];
    let nb_nodes : &str = &args[3];
    let walltime : &str = &args[4];
    
    let job : JobSubmitResponse = reserve_node(username, password, nb_nodes, walltime).unwrap();
    delete_job(username, password, job.uid.to_string());
    // get_grid5000(username, password);
}

fn reserve_node(username : &str, password : &str, nb_nodes : &str, walltime : &str) -> Result<JobSubmitResponse, reqwest::Error> {

    let api_url = "https://api.grid5000.fr/3.0/sites/rennes/jobs/?pretty";

    let mut map = HashMap::new();
    let resource = format!("nodes={},walltime={}", nb_nodes, walltime); 
    map.insert("resources", resource.as_str());
    map.insert("command", "sleep 7200");

    let client = reqwest::blocking::Client::new();


    let res = client.post(api_url)
                                 .json(&map)
                                 .basic_auth(username, Some(password))
                                 .send()
                                 .expect("Failed to send request");

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let response_body : JobSubmitResponse = res.json().unwrap();
    println!("{:?}", response_body);

    Ok(response_body)
}



/*fn deploy_env_on_node(username : &str, password : &str, nodes : Vec<&str>, environment : &str, ssh_key : &str) -> Result<(), reqwest::Error>  {

    let api_url = "https://api.grid5000.fr/3.0/sites/rennes/deployments";

    let mut map = HashMap::new();
    map.insert("nodes", serde_json::to_string(&nodes).unwrap());
    map.insert("environment", environment);
    map.insert("key", ssh_key);

    let client = reqwest::blocking::Client::new();

    let res = client.post(api_url)
                                 .json(&map)
                                 .basic_auth(username, Some(password))
                                 .send()
                                 .expect("Failed to send request");

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{:#?}", res.text());

    Ok(())
}*/

fn get_grid5000(username : &str, password : &str) -> Result<(), reqwest::Error> {
    
    let client = reqwest::blocking::Client::new();
    let res = client.get("https://api.grid5000.fr/3.0/?pretty")
                    .basic_auth(username, Some(password))
                    .send()
                    .expect("Failed to send request");

    // Move and borrow value of `res`
    let body = res.text().unwrap();
    println!("Body:\n{}", body);

    Ok(())
}

fn delete_job(username : &str, password : &str, job_to_delete : String) -> Result<(), reqwest::Error> {

    let api_url = "https://api.grid5000.fr/3.0/sites/rennes/jobs/";

    let client = reqwest::blocking::Client::new();
    let res = client.delete(format!("{}{}", api_url, job_to_delete).as_str())
                    .basic_auth(username, Some(password))
                    .send()
                    .expect("Failed to send request");

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    // Move and borrow value of `res`
    let response_body = res.text().unwrap();
    println!("Body:\n{}", response_body);
                    
    Ok(())
}