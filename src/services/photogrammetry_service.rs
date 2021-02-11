extern crate reqwest;

use super::service_access_information::ServiceAccessInformation;

/// Rest client for the photogrammetry service*
#[allow(dead_code)]
pub struct PhotogrammetryService {
    access_information: ServiceAccessInformation,
}

impl PhotogrammetryService {
    #[allow(dead_code)]
    pub fn new(access_information: ServiceAccessInformation) -> PhotogrammetryService {
        PhotogrammetryService { access_information: access_information }
    }

    #[allow(dead_code)]
    pub fn start_job() {
        todo!();
    }

    #[allow(dead_code)]
    pub fn get_job_info() {
        todo!();
    }

    /// Displays information about how to access the web service
    #[allow(dead_code)]
    pub fn print_access_info(&self){
        println!("host: {}", self.access_information.get_host());
        println!("port: {}", self.access_information.get_port());

        let username = self.access_information.get_username();
        if !username.is_empty() {
            println!("username: {}", self.access_information.get_username());
        }

        let pwd = self.access_information.get_password();
        if !pwd.is_empty() {
            println!("password: *****");
        }
    }
}
