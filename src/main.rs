mod services;
mod clusters;

use services::service_access_information::*;
use services::photogrammetry_service::*;
use crate::clusters::grid5000::Grid5000;
use crate::clusters::cluster::Cluster;

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

    let cluster = Grid5000::new("test");
    println!("{}",cluster.has_green_energy_available());
}

