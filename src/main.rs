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

    let meteo_client = MeteoClient::new();
    let grid5000_meteo_array = meteo_client.get_weather_for_grid5000_sites();
    print!("{:?}", grid5000_meteo_array);
}