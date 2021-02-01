mod services;
mod clusters;
mod ssh_client;

use crate::clusters::grid5000::Grid5000;
use crate::clusters::grid5000::grid5000_client_struct::*;

// use crate::clusters::cluster::Cluster;

// use services::service_access_information::*;
// use services::photogrammetry_service::*;
use std::{env, path::Path};
use ssh_client::SshClient;

#[tokio::main]
async fn main() {

    /*let photogrammetry_access_info = ServiceAccessInformation::new(
        String::from("myUrl"),
        8080,
        String::from(""),
        String::from(""),
    );

    let photogrammetry_service = PhotogrammetryService::new(photogrammetry_access_info);

    photogrammetry_service.print_access_info();*/

    let cluster = Grid5000::new();

    // println!("{}",&cluster.has_green_energy_available());

    let args: Vec<String> = env::args().collect();

    let username : &str = &args[1];
    let password : &str = &args[2];
    let site : &str = &args[3];
    let nb_nodes : &str = &args[4];
    let walltime : &str = &args[5];
    let ssh_key_path : &str = &args[6];

    let job_submit_response : JobSubmitResponse = cluster.make_reservation(username, password, site, nb_nodes, walltime, ssh_key_path);

    let node = &job_submit_response.assigned_nodes[0];

    let tcp_address = node.as_str();
    let username = "root";
    let pub_key: &Path = Path::new("C:\\Users\\Bart\\.ssh\\test.pub");
    let priv_key: &Path = Path::new("C:\\Users\\Bart\\.ssh\\test.pem");

    let ssh_client : SshClient = SshClient::new(tcp_address, username, pub_key, priv_key);

    ssh_client.install_docker_git(tcp_address, username, pub_key, priv_key);
    ssh_client.clone_git_repo(tcp_address, username, pub_key, priv_key);
    ssh_client.build_photo_docker(tcp_address, username, pub_key, priv_key);
    ssh_client.run_docker(tcp_address, username, pub_key, priv_key);
}