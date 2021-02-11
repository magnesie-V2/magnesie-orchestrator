#[allow(dead_code)]
#[allow(unused_imports)]
extern crate reqwest;
extern crate serde;

pub mod grid5000_client_struct;

#[allow(unused_imports)]
use std::{env, thread, time, path::{PathBuf}};
use std::fs;

use chrono::{Timelike, Utc};

use grid5000_client_struct::*;

#[allow(unused_imports)]
use crate::ssh_client::SshClient;

pub struct Grid5000 {
    api_base_url: &'static str,
    deploy_url: &'static str,
    job_url_pretty: &'static str,
    job_url: &'static str,
    username : String,
    password : String,
    site : String,
    nb_nodes : String,
    walltime : String,
    ssh_key_path : String
}

impl Grid5000 {

    /**
        username : username Grid5000
        password : password Grid5000
        site : le site sur lequel on souhaite déployer le service de photogrammetry (nantes, rennes, nacy etc...)
        walltime : le temps de réservation des nodes, en heures
        ssh_key_path : le chemin vers la clé publique à utilsier pour la réservation. 
    */
    #[allow(dead_code)]
    pub fn new(username: String, password: String, site: String, walltime: String, ssh_key_path: String) -> Grid5000 {
        Grid5000 {
            api_base_url: "https://api.grid5000.fr/3.0/sites/",
            deploy_url: "/deployments/",
            job_url_pretty: "/jobs/?pretty/",
            job_url: "/jobs/",
            username,
            password,
            site,
            nb_nodes : String::from("1"),
            walltime,
            ssh_key_path
        }
    }

    #[allow(dead_code)]
    pub fn has_green_energy_available(self) -> bool {
        let now = Utc::now();
        let minute = now.minute();
        return if minute % 2 == 0 { true } else { false };
    }

    #[allow(dead_code)]
    // Make a reservartio nand return the adress of the reserved node
    pub fn make_reservation(&self) -> String {

        let env : String = String::from("debian10-x64-min");

        let ssh_key: String = self.get_ssh_key().unwrap();

        // Reserve a node and get the resposne from API
        let job_waiting: JobSubmitResponse =
            self.reserve_node().unwrap();

        // Check if the job's reservation is finished
        let mut job_deployed: JobSubmitResponse =
            self.get_reservation(job_waiting.uid.to_string()).unwrap();

        while job_deployed.state != "running" {
            job_deployed = self.get_reservation(job_waiting.uid.to_string()).unwrap();
        }

        // When job is reserved, deploy environment on node
        let mut deploy_env_response : DeployEnvResponse = self.deploy_env_on_node(&job_deployed.assigned_nodes,env, ssh_key).unwrap();

        while deploy_env_response.status != "terminated" && deploy_env_response.status != "error" && deploy_env_response.status != "canceled" {
            deploy_env_response = self.get_deployment(deploy_env_response.uid).unwrap();
        }

        return job_deployed.assigned_nodes.remove(0);
    }

    // Delete reservation of node with uid = job_uid
    #[allow(dead_code)]
    pub fn delete_reservation(&self, job_to_delete: String) -> Result<(), reqwest::Error> {
        let api_url = format!("{}{}{}", self.api_base_url, self.site, self.job_url);

        let client = reqwest::blocking::Client::new();
        let res = client
            .delete(format!("{}{}", api_url, job_to_delete).as_str())
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .expect("Failed to send request");

        // Move and borrow value of `res`
        let response_body = res.text().unwrap();
        println!("{:?}", response_body);
        println!("");

        Ok(())
    }

    fn reserve_node(&self) -> Result<grid5000_client_struct::JobSubmitResponse, reqwest::Error> {
        let api_url = format!("{}{}{}", self.api_base_url, self.site, self.job_url_pretty);

        let mut deploy_option: Vec<String> = Vec::new();
        deploy_option.push(String::from("deploy"));

        let resource = format!("nodes={},walltime={}", self.nb_nodes, self.walltime);

        let request_body = ReservationRequest {
            name: String::from("test_magnes.ie"),
            resources: resource,
            command: String::from("sleep 7200"),
            types: deploy_option,
        };

        let client = reqwest::blocking::Client::new();

        let res = client
            .post(api_url.as_str())
            .json(&request_body)
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .expect("Failed to send request");

        let response_body: JobSubmitResponse = res.json().unwrap();
        println!("{:?}", response_body);
        println!("");

        Ok(response_body)
    }

    // Check state of reservation with uid = job_uid
    fn get_reservation(&self, job_uid: String) -> Result<JobSubmitResponse, reqwest::Error> {
        thread::sleep(time::Duration::from_secs(5));

        let api_url = format!("{}{}{}", self.api_base_url, self.site, self.job_url,);

        let client = reqwest::blocking::Client::new();

        let res = client
            .get(format!("{}{}", api_url, job_uid).as_str())
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .expect("Failed to send request");

        let response_body: JobSubmitResponse = res.json().unwrap();
        println!("{:?}", response_body);
        println!("");

        Ok(response_body)
    }

    // Deploy provided environment to specified node
    fn deploy_env_on_node(&self, target_nodes: &Vec<String>, environment: String, ssh_key: String) -> Result<DeployEnvResponse, reqwest::Error> {
        let api_url = format!("{}{}{}", self.api_base_url, self.site, self.deploy_url);

        let request_body = DeploymentRequest {
            nodes: target_nodes.clone(),
            environment: environment,
            key: ssh_key,
        };

        let client = reqwest::blocking::Client::new();

        let res = client
            .post(api_url.as_str())
            .json(&request_body)
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .expect("Failed to send request");

        let response_body: DeployEnvResponse = res.json().unwrap();
        println!("{:?}", response_body);
        println!("");

        Ok(response_body)
    }

    fn get_deployment(&self, deployment_uid: String) -> Result<DeployEnvResponse, reqwest::Error> {
        thread::sleep(time::Duration::from_secs(60));

        let api_url = format!("{}{}{}", self.api_base_url, self.site, self.deploy_url);

        let client = reqwest::blocking::Client::new();

        let res = client
            .get(format!("{}{}", api_url, deployment_uid).as_str())
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .expect("Failed to send request");

        let response_body: DeployEnvResponse = res.json().unwrap();
        println!("{:?}", response_body);
        println!("");

        Ok(response_body)
    }

    // Get the SSH key from provided file
    fn get_ssh_key(&self) -> Result<String, Box<dyn std::error::Error + 'static>> {
        println!("{}", &self.ssh_key_path);
        let ssh_key: String = fs::read_to_string(&self.ssh_key_path)?;
        Ok(ssh_key)
    }
}

#[test]
fn launch_grid5000_client() {
    
    let args: Vec<String> = env::args().collect();

    let username : &str = &args[2];
    let password : &str = &args[3];
    let site : &str = &args[4];
    let walltime : &str = &args[5];
    let ssh_key_path : &str = &args[6];

    let cluster = Grid5000::new(String::from(username),
                                        String::from(password),
                                        String::from(site),
                                        String::from(walltime),
                                        String::from(ssh_key_path));


    let reserved_node : String = format!("{}{}", cluster.make_reservation(), ":22"); 

    let username : String = String::from("root");
    let pub_key: PathBuf = PathBuf::from("C:\\Users\\Bart\\.ssh\\orchestrateur_key.pub");
    let priv_key: PathBuf = PathBuf::from("C:\\Users\\Bart\\.ssh\\orchestrateur_key.pem");

    // println!("{}", reserved_node);

    let ssh_client : SshClient = SshClient::new(reserved_node, username, pub_key, priv_key);

    ssh_client.install_docker_git();
    ssh_client.git_clone_mock_repo();
    ssh_client.build_photo_docker();
    ssh_client.run_docker();
}