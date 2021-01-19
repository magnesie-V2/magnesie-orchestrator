mod services;

use std::sync::{Arc, Mutex};

use services::photogrammetry_service::PhotogrammetryService;
use services::services_keeper::ServicesKeeper;
use services::service_access_information::ServiceAccessInformation;
use crate::services::service_error::ServiceError;

#[allow(dead_code)]
fn main() {
    let services_keeper = Arc::new(Mutex::new(ServicesKeeper::new()));

    let photogrammetry_access_info = ServiceAccessInformation::new(
        String::from(String::from("8645cc99-fdca-4a6a-bf45-1eb639a54f2c.mock.pstmn.io")), // 8645cc99-fdca-4a6a-bf45-1eb639a54f2c.mock.pstmn.io           172.17.0.1
        80,
        String::from(""),
        String::from(""),
    );

    services_keeper.lock().unwrap().register_service("photogrammetry", photogrammetry_access_info);

    match PhotogrammetryService::test(services_keeper.clone()) {
        Ok(_) => {}
        Err(error) => println!("{}", error)
    }

    loop{} // preventing the app to terminate to avoid having to join services threads with the main thread
}

