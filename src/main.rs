mod services;
mod clusters;
mod ssh_client;

use crate::clusters::grid5000::Grid5000;

// use crate::clusters::cluster::Cluster;

// use services::service_access_information::*;
// use services::photogrammetry_service::*;
use std::{env, path::{PathBuf}};
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

    let args: Vec<String> = env::args().collect();

    let username : &str = &args[1];
    let password : &str = &args[2];
    let site : &str = &args[3];
    let nb_nodes : &str = &args[4];
    let walltime : &str = &args[5];
    let ssh_key_path : &str = &args[6];

    let cluster = Grid5000::new(username.to_string(),
                                        password.to_string(),
                                        site.to_string(),
                                        nb_nodes.to_string(),
                                        walltime.to_string(),
                                        ssh_key_path.to_string());

    // println!("{}",&cluster.has_green_energy_available());


    let reserved_node : String = cluster.make_reservation();

    let username : String = "root".to_string();
    let pub_key: PathBuf = PathBuf::from("C:\\Users\\Bart\\.ssh\\orchestrateur_key.pub");
    let priv_key: PathBuf = PathBuf::from("C:\\Users\\Bart\\.ssh\\orchestrateur_key.pem");

    println!("{}", reserved_node);

    let ssh_client : SshClient = SshClient::new(reserved_node, username, pub_key, priv_key);

    ssh_client.install_docker_git();
    ssh_client.clone_git_repo();
    ssh_client.build_photo_docker();
    ssh_client.run_docker();
}