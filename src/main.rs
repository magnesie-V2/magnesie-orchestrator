mod services;

use services::service_access_information::*;
use services::photogrammetry_service::*;

#[tokio::main]
async fn main() {

    let photogrammetry_access_info = ServiceAccessInformation::new(
        String::from("myUrl"),
        8080,
        String::from(""),
        String::from(""),
    );

    let photogrammetry_service = PhotogrammetryService::new(photogrammetry_access_info);

    photogrammetry_service.print_access_info();
}

