use serde::Deserialize;
use serde::Serialize;

use super::service_access_information::ServiceAccessInformation;
use super::service_error::ServiceError;
use reqwest::blocking::Client;

#[derive(Deserialize, Debug)]
pub struct PhotogrammetryJob {
    pub id: Option<u8>,
    pub status: Option<String>,
    pub result: Option<String>
}

#[derive(Serialize, Debug)]
pub struct PhotogrammetryJobRequestBody{
    pub photos: Vec<String>,
    pub callback: String
}

/// Rest client for the photogrammetry service
pub struct PhotogrammetryService {
    access_information: ServiceAccessInformation,
    client: Client // it's best to create a client and reuse it for request pooling
}

#[allow(dead_code)]
impl PhotogrammetryService {
    pub fn new(access_information: ServiceAccessInformation) -> PhotogrammetryService {

        PhotogrammetryService { access_information, client: reqwest::blocking::Client::new() }
    }

    /// Sends pictures urls to the photogrammetry service and returns the id of the created job
    pub fn create_job(&self, pictures_urls: Vec<String>, callback_url: String) -> Result<u8, ServiceError> {
        let request_url = format!("http://{host}:{port}/job",
                                  host=self.access_information.get_host(),
                                  port=self.access_information.get_port());

        let body = PhotogrammetryJobRequestBody {
            photos: pictures_urls,
            callback: callback_url
        };

        let request = self.client.post(&request_url).json(&body);

        let response = request.send()?;

        let response_body: PhotogrammetryJob = response.json()?;

        match response_body.id {
            None => {
                let error_message = String::from("The id field wasn't found in the response body");
                Err(ServiceError::from(error_message))
            },
            Some(id) => Ok(id)
        }
    }

    pub fn get_job(&self, id: u8) -> Result<PhotogrammetryJob, ServiceError>{
        let request_url = format!("http://{host}:{port}/job/{id}",
                                  host=self.access_information.get_host(),
                                  port=self.access_information.get_port(),
                                  id=id);

        let request = self.client.get(&request_url);

        let response = request.send()?;
        let mut response_body: PhotogrammetryJob = response.json()?;

        match response_body.id {
            None => response_body.id = Some(id),
            _ => {}
        }

        Ok(response_body)
    }

    /// Displays information about how to access the web service
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







