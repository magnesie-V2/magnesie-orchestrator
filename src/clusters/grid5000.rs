#[allow(dead_code)]
#[allow(unused_imports)]
extern crate reqwest;
extern crate serde;

use super::grid5000_struct::*;
use crate::ssh_client::SshClient;
use crate::meteo_service::MeteoClient;

use crate::clusters::{Cluster, ClusterError, ReservationStatus};
use crate::services::ServiceAccessInformation;

#[allow(unused_imports)]
use std::{env, 
          thread, 
          time,
          fs, 
          time::{SystemTime, UNIX_EPOCH},
          path::{Path, PathBuf}, 
          io::BufReader, 
          fs::File};

use chrono::{Timelike, Utc};

use rand::Rng;

#[allow(unused_imports)]

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
    ssh_key_path : String,
    reserved_node_address: String,
    uid : String
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
    pub fn new(username: String, password: String, site: String, walltime: String) -> Grid5000 {
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
            ssh_key_path : String::from("config/orchestrateur_key.pub"),
            reserved_node_address : String::from(""),
            uid : String::from(""),
        }
    }

    #[allow(dead_code)]
    pub fn new_random_site(username: String, password: String, walltime: String) -> Grid5000 {
        
        let mut ret = Grid5000 {
            api_base_url: "https://api.grid5000.fr/3.0/sites/",
            deploy_url: "/deployments/",
            job_url_pretty: "/jobs/?pretty/",
            job_url: "/jobs/",
            site : String::new(),
            username,
            password,
            nb_nodes : String::from("1"),
            walltime,
            ssh_key_path : String::from("config/orchestrateur_key.pub"),
            reserved_node_address : String::from(""),
            uid : String::from(""),
        };

        ret.site = ret.choose_random_site_with_green_energy();

        return ret;
    }

    #[allow(dead_code)]
    pub fn has_green_energy_available(self) -> bool {
        let now = Utc::now();
        let minute = now.minute();
        return if minute % 2 == 0 { true } else { false };
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

    fn reserve_node(&self) -> Result<JobSubmitResponse, reqwest::Error> {
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

    #[allow(dead_code)]
    // Make a reservartio nand return the adress of the reserved node
    fn make_reservation(&mut self) -> String {

        let env : String = String::from("debian10-x64-min");

        let ssh_key: String = self.get_ssh_key().unwrap();

        // Reserve a node and get the resposne from API
        let job_waiting: JobSubmitResponse = self.reserve_node().unwrap();

        self.uid = job_waiting.uid.to_string();

        // Check if the job's reservation is finished
        let mut job_deployed: JobSubmitResponse = self.get_reservation(self.uid.clone()).unwrap();

        while job_deployed.state != "running" {
            job_deployed = self.get_reservation(self.uid.clone()).unwrap();
        }

        // When job is reserved, deploy environment on node
        let mut deploy_env_response : DeployEnvResponse = self.deploy_env_on_node(&job_deployed.assigned_nodes,env, ssh_key).unwrap();

        while deploy_env_response.status != "terminated" && deploy_env_response.status != "error" && deploy_env_response.status != "canceled" {
            deploy_env_response = self.get_deployment(deploy_env_response.uid).unwrap();
        }

        return job_deployed.assigned_nodes.remove(0);
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

    #[allow(dead_code)]
    fn get_sites_with_green_energy(&self) -> Vec<String> {
        /*let now = Utc::now();
        let minute = now.minute();
        return if minute % 2 == 0 { true } else { false };*/

        let start = SystemTime::now();
        let now = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards").as_secs();

        let meteo_client : MeteoClient = MeteoClient::new();
        let grid5000_meteo_array : Vec<(String,(u64, f64, u64))> = meteo_client.get_weather_for_grid5000_sites();
        
        let ret : Vec<String> = grid5000_meteo_array.into_iter()
                                                                        .filter(|x : &(String, (u64, f64, u64)) | (x.1.0 == 800 && now < x.1.2) || x.1.1 > 4.2)
                                                                        .map(|x| x.0)
                                                                        .collect();
        
        return ret;        
    }

    #[allow(dead_code)]
    fn choose_random_site_with_green_energy(&self) -> String {

        let mut available_sites : Vec<String> = self.get_sites_with_green_energy();

        let rand_num = rand::thread_rng().gen_range(0..available_sites.len());

        return available_sites.remove(rand_num);

    }
}

impl Cluster for Grid5000 {

    fn deploy_photogrammetry_service(&mut self) -> Result<ServiceAccessInformation, ClusterError> {
        
        self.reserved_node_address = self.make_reservation();

        let node_username : String = String::from("root");
    
        let pub_key_path: PathBuf = PathBuf::from(&self.ssh_key_path);

        let priv_key: PathBuf = PathBuf::from("config/orchestrateur_key.pem");
        let ssh_client : SshClient = SshClient::new(self.reserved_node_address.clone(), node_username, pub_key_path, priv_key);

        ssh_client.install_docker();
        ssh_client.pull_photo_docker();
        ssh_client.run_docker();
        
        Ok(ServiceAccessInformation::new(
            &self.reserved_node_address,
            7879,
            &self.username,
            &self.password
        ))
    }

    fn get_access_information(&self) -> Option<ServiceAccessInformation> {
        Some(ServiceAccessInformation::new(
            &self.reserved_node_address,
            7879,
            &self.username,
            &self.password
        ))
    }

    fn get_reservation_status(&self) -> Option<ReservationStatus> {
        let job_deployed: JobSubmitResponse = self.get_reservation(self.uid.clone()).unwrap();
        if job_deployed.state == "waiting" || job_deployed.state == "launching" || job_deployed.state == "hold" {
            return Some(ReservationStatus::Pending)
        }
        else if job_deployed.state == "running" {
            return Some(ReservationStatus::ResourcesAvailable)
        }
        else {
            return Some(ReservationStatus::Expired)
        }
    }

}

#[test]
fn launch_grid5000_client() {
    
    let args: Vec<String> = env::args().collect();

    let username : &str = &args[2];
    let password : &str = &args[3];
    let site : &str = &args[4];
    let walltime : &str = &args[5];
    
    let node_username : String = String::from("root");
    
    let pub_key : String = String::from("config/orchestrateur_key.pub");
    let pub_key_path: PathBuf = PathBuf::from(&pub_key);

    let priv_key: PathBuf = PathBuf::from("config/orchestrateur_key.pem");

    /*let cluster = Grid5000::new_random_site(String::from(username),
                                        String::from(password),
                                        String::from(walltime),
                                        pub_key);*/

    let cluster = Grid5000::new(String::from(username),
                                String::from(password),
                                String::from(site),
                                String::from(walltime));

    println!("Attempting reservation on site {}", &cluster.site);

    let reserved_node : String = cluster.make_reservation(); 


    println!("{}", reserved_node);

    let ssh_client : SshClient = SshClient::new(reserved_node, node_username, pub_key_path, priv_key);

    ssh_client.install_docker();
    ssh_client.pull_photo_docker();
    ssh_client.run_docker();
}