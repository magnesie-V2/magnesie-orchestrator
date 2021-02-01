#[allow(dead_code)]
extern crate reqwest;
extern crate serde;

mod grid5000_client_struct;

use std::{thread, time};
use std::fs;

use chrono::{Timelike, Utc};

use grid5000_client_struct::*;

pub struct Grid5000 {
    api_base_url: &'static str,
    deploy_url: &'static str,
    job_url_pretty: &'static str,
    job_url: &'static str,
}

impl Grid5000 {
    pub fn new(_: &str) -> Grid5000 {
        Grid5000 {
            api_base_url: "https://api.grid5000.fr/3.0/sites/",
            deploy_url: "/deployments/",
            job_url_pretty: "/jobs/?pretty/",
            job_url: "/jobs/",
        }
    }

    pub fn has_green_energy_available(self) -> bool {
        let now = Utc::now();
        let minute = now.minute();
        return if minute % 2 == 0 { true } else { false };
    }

    #[allow(dead_code)]
    pub fn make_reservation(&self, username: &str, password: &str, site: &str, nb_nodes: &str, walltime: &str, ssh_key_path: &str) {

        let env = "debian10-x64-min";

        let ssh_key: String = self.get_ssh_key(ssh_key_path).unwrap();

        // Reserve a node and get the resposne from API
        let job_waiting: JobSubmitResponse =
            self.reserve_node(username, password, site, nb_nodes, walltime).unwrap();

        // Check if the job's reservation is finished
        let mut job_deployed: JobSubmitResponse =
            self.get_reservation(username, password, site, job_waiting.uid.to_string()).unwrap();

        while job_deployed.state != "running" {
            job_deployed =
                self.get_reservation(username, password, site, job_waiting.uid.to_string()).unwrap();
        }

        // When job is reserved, deploy environment on node
        self.deploy_env_on_node(
            username,
            password,
            site,
            job_deployed.assigned_nodes,
            env,
            ssh_key.as_str(),
        ).unwrap();
    }

    // Delete reservation of node with uid = job_uid
    #[allow(dead_code)]
    pub fn delete_reservation(&self, username: &str, password: &str, site: &str, job_to_delete: String) -> Result<(), reqwest::Error> {
        let api_url = format!("{}{}{}", self.api_base_url, site, self.job_url);

        let client = reqwest::blocking::Client::new();
        let res = client
            .delete(format!("{}{}", api_url, job_to_delete).as_str())
            .basic_auth(username, Some(password))
            .send()
            .expect("Failed to send request");

        // Move and borrow value of `res`
        let response_body = res.text().unwrap();
        println!("{:?}", response_body);
        println!("");

        Ok(())
    }

    fn reserve_node(&self, username: &str, password: &str, site: &str, nb_nodes: &str, walltime: &str) -> Result<grid5000_client_struct::JobSubmitResponse, reqwest::Error> {
        let api_url = format!("{}{}{}", self.api_base_url, site, self.job_url_pretty);

        let mut deploy_option: Vec<String> = Vec::new();
        deploy_option.push("deploy".to_string());

        let resource = format!("nodes={},walltime={}", nb_nodes, walltime);

        let request_body = ReservationRequest {
            name: "test_magnes.ie".to_string(),
            resources: resource,
            command: "sleep 7200".to_string(),
            types: deploy_option,
        };

        let client = reqwest::blocking::Client::new();

        let res = client
            .post(api_url.as_str())
            .json(&request_body)
            .basic_auth(username, Some(password))
            .send()
            .expect("Failed to send request");

        let response_body: JobSubmitResponse = res.json().unwrap();
        println!("{:?}", response_body);
        println!("");

        Ok(response_body)
    }

    // Check state of reservation with uid = job_uid
    fn get_reservation(&self, username: &str, password: &str, site: &str, job_uid: String) -> Result<JobSubmitResponse, reqwest::Error> {
        thread::sleep(time::Duration::from_secs(5));

        let api_url = format!("{}{}{}", self.api_base_url, site, self.job_url,);

        let client = reqwest::blocking::Client::new();

        let res = client
            .get(format!("{}{}", api_url, job_uid).as_str())
            .basic_auth(username, Some(password))
            .send()
            .expect("Failed to send request");

        let response_body: JobSubmitResponse = res.json().unwrap();
        println!("{:?}", response_body);
        println!("");

        Ok(response_body)
    }

    // Deploy provided environment to specified node
    fn deploy_env_on_node(&self, username: &str, password: &str, site: &str, target_nodes: Vec<String>, environment: &str, ssh_key: &str) -> Result<(), reqwest::Error> {
        let api_url = format!("{}{}{}", self.api_base_url, site, self.deploy_url);

        let request_body = DeploymentRequest {
            nodes: target_nodes,
            environment: environment.to_string(),
            key: ssh_key.to_string(),
        };

        let client = reqwest::blocking::Client::new();

        let res = client
            .post(api_url.as_str())
            .json(&request_body)
            .basic_auth(username, Some(password))
            .send()
            .expect("Failed to send request");

        let response_body: DeployEnvResponse = res.json().unwrap();
        println!("{:?}", response_body);
        println!("");

        Ok(())
    }

    // Get the SSH key from provided file
    fn get_ssh_key(&self, file_path: &str) -> Result<String, Box<dyn std::error::Error + 'static>> {
        let ssh_key: String = fs::read_to_string(file_path)?;
        Ok(ssh_key)
    }
}
