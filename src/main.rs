mod services;

use services::photogrammetry_service::PhotogrammetryService;
use services::service_access_information::ServiceAccessInformation;

#[allow(dead_code)]
fn main() {
    let photogrammetry_access_info = ServiceAccessInformation::new(
        String::from(String::from("8645cc99-fdca-4a6a-bf45-1eb639a54f2c.mock.pstmn.io")),
        80,
        String::from(""),
        String::from(""),
    );

    match PhotogrammetryService::test(photogrammetry_access_info) {
        Ok(_) => {}
        Err(error) => println!("{}", error)
    }

    loop{} // preventing the app to terminate to avoid having to join services threads with the main thread
}

