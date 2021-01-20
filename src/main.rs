mod services;

use std::sync::{Arc, RwLock};

use services::photogrammetry_service::PhotogrammetryService;
use services::services_keeper::ServicesKeeper;
use services::service_access_information::ServiceAccessInformation;

#[allow(dead_code)]
fn main() {
    let services_keeper = Arc::new(RwLock::new(ServicesKeeper::new()));

    let photogrammetry_access_info = ServiceAccessInformation::new(
        String::from(String::from("172.17.0.1")), // 8645cc99-fdca-4a6a-bf45-1eb639a54f2c.mock.pstmn.io           172.17.0.1
        7979, // 80             7979
        String::from(""),
        String::from(""),
    );

    match PhotogrammetryService::new(services_keeper.clone()) {
        Ok(service) => {
            match service.test() {
                Ok(_) => {}
                Err(error) => println!("{}", error)
            }

            println!("Setting the spg info");
            services_keeper.write().unwrap().register_service("photogrammetry", photogrammetry_access_info);
            println!("spg info set");

            match service.test() {
                Ok(_) => {}
                Err(error) => println!("{}", error)
            }
        },
        Err(_) => unimplemented!(),
    };

    loop{} // preventing the app to terminate to avoid having to join services threads with the main thread
}

