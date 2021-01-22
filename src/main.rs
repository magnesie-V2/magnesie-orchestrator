mod services;
mod jobs_buffer;

use std::sync::{Arc, RwLock};

use services::photogrammetry::PhotogrammetryService;
use services::remote::{ServicesKeeper, ServiceAccessInformation};
use jobs_buffer::{JobsBuffer, BufferedJob};

#[allow(dead_code)]
fn main() {
    let services_keeper = Arc::new(RwLock::new(ServicesKeeper::new()));
    let jobs_buffer = Arc::new(RwLock::new(JobsBuffer::new()));

    let photogrammetry_access_info = ServiceAccessInformation::new(
        "172.17.0.1", // 8645cc99-fdca-4a6a-bf45-1eb639a54f2c.mock.pstmn.io           172.17.0.1
        7979, // 80             7979
        "",
        "",
    );

    match PhotogrammetryService::new(services_keeper.clone()) {
        Ok(service) => {
            println!("Test: using the photogrammetry service without any cluster selection and service deployment");
            match service.test() {
                Ok(_) => {}
                Err(error) => println!("{}", error)
            }

            println!("Setting the photogrammetry service information (mock of cluster selection)");
            services_keeper.write().unwrap().register_service("photogrammetry", photogrammetry_access_info);

            println!("Test: using the photogrammetry service");
            match service.test() {
                Ok(_) => {}
                Err(error) => println!("{}", error)
            }
        },
        Err(_) => unimplemented!(),
    };

    loop{} // preventing the app to terminate to avoid having to join services threads with the main thread
}

