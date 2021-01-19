mod services;

use services::photogrammetry_service::PhotogrammetryService;
use services::services_keeper::ServicesKeeper;
use services::service_access_information::ServiceAccessInformation;

#[allow(dead_code)]
fn main() {
    let mut services_keeper = ServicesKeeper::new();

    let photogrammetry_access_info = ServiceAccessInformation::new(
        String::from(String::from("localhost")), // 8645cc99-fdca-4a6a-bf45-1eb639a54f2c.mock.pstmn.io
        80,
        String::from(""),
        String::from(""),
    );

    services_keeper.register_service("photogrammetry", photogrammetry_access_info);

    match PhotogrammetryService::test(services_keeper) {
        Ok(_) => {}
        Err(error) => println!("{}", error)
    }
    /*photogrammetry_access_info = ServiceAccessInformation::new(
        String::from(String::from("fake-address")),
        80,
        String::from(""),
        String::from(""),
    );

    match PhotogrammetryService::test(&services_keeper) {
        Ok(_) => {}
        Err(error) => println!("{}", error)
    }
    */

    loop{} // preventing the app to terminate to avoid having to join services threads with the main thread
}

