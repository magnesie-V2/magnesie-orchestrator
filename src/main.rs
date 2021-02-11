mod services;
mod clusters;
mod ssh_client;
mod meteo_service;

use std::{env};
use meteo_service::MeteoClient;

// use crate::clusters::cluster::Cluster;

// use services::service_access_information::*;
// use services::photogrammetry_service::*;

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

    // println!("{}",&cluster.has_green_energy_available());

    let args: Vec<String> = env::args().collect();
    let api_key : &str = &args[1];
    // let city : &str = &args[2];

    let meteo_client = MeteoClient::new(api_key.to_string());
    meteo_client.get_weather_for_grid5000_sites();
}