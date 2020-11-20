mod services;

use services::service_access_information::*;
use services::photogrammetry_service::*;

#[allow(dead_code)]
fn main() {

    let photogrammetry_access_info = ServiceAccessInformation::new(
        String::from("8645cc99-fdca-4a6a-bf45-1eb639a54f2c.mock.pstmn.io"),
        80,
        String::from(""),
        String::from(""),
    );
    let photogrammetry_service = PhotogrammetryService::new(photogrammetry_access_info);

    let mock_photos = [
        String::from("photo1.jpeg"),
        String::from("photo2.jpeg"),
        String::from("photo3.jpeg")
    ].to_vec();
    let photogrammetry_callback = String::from("orchestrator/photogrammetry-callback");

    match photogrammetry_service.create_job(mock_photos, photogrammetry_callback) {
        Ok(id) => println!("Created job of id: {}", id),
        Err(error) => println!("{}", error)
    }

    match photogrammetry_service.get_job(0) {
        Ok(job) => println!("Job of id {} is currently: {}", job.id.unwrap(), job.status.unwrap()),
        Err(error) => println!("{}", error)
    }
}

