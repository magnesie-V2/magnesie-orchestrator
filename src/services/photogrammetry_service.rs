use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;
use std::io::{Read, Write};

use reqwest::blocking::Client;
use serde::Deserialize;
use serde::Serialize;

use super::service_access_information::ServiceAccessInformation;
use super::service_error::ServiceError;

#[derive(Deserialize, Debug)]
pub struct PhotogrammetryJob {
    pub id: Option<String>,
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
    pub fn new(access_information: ServiceAccessInformation) -> Result<Arc<PhotogrammetryService>, ServiceError> {
        let callback_listener = TcpListener::bind("localhost:7878")?;

        let service = Arc::new(PhotogrammetryService {
            access_information,
            client: reqwest::blocking::Client::new()
        });

        let service_clone = service.clone();
        thread::spawn(move || -> Result<String,ServiceError> {
            for stream in callback_listener.incoming() {
                match service_clone.handle_connection(stream?) {
                    Ok(_) => {}
                    Err(error) => println!("{}", error)
                }
            }
            Ok(String::from(""))
        });

        Ok(service)
    }

    /// Sends a job creation requests and asks for information about it
    pub fn test(photogrammetry_access_info: ServiceAccessInformation) -> Result<bool, ServiceError>{
        let photogrammetry_service = PhotogrammetryService::new(photogrammetry_access_info)?;

        let mock_photos = [
            String::from("photo1.jpeg"),
            String::from("photo2.jpeg"),
            String::from("photo3.jpeg")
        ].to_vec();
        let photogrammetry_callback = String::from("orchestrator/photogrammetry-callback");

        let id = photogrammetry_service.create_job(mock_photos, photogrammetry_callback)?;
        println!("Created job of id: {}", id);

        let job = photogrammetry_service.get_job(id)?;
        println!("Job of id {} is currently: {}", job.id.unwrap(), job.status.unwrap());

        Ok(true) // No error thrown so the test returns true
    }

    /// Sends pictures urls to the photogrammetry webservice and returns the id of the created job
    pub fn create_job(&self, pictures_urls: Vec<String>, callback_url: String) -> Result<String, ServiceError> {
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

    /// Retrieves information about a job based on its id
    pub fn get_job(&self, id: String) -> Result<PhotogrammetryJob, ServiceError>{
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

    /// Displays information about how to access the webservice
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

    /// TODO Error cases
    pub fn handle_connection (&self, mut stream: TcpStream) -> Result<(), ServiceError> {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let buffer_as_string = String::from(std::str::from_utf8(&buffer)?);
        let mut request_terms = buffer_as_string.split_whitespace();

        let method = match request_terms.next() {
            Some(x) => x,
            None => unimplemented!(),
        };

        let mut path= "";
        match request_terms.next() {
            Some(x) => path = x,
            None => {}
        };

        let mut path_terms = path.split("/");
        match path_terms.next() {
            Some(x) => x,
            None => unimplemented!(),
        };
        match path_terms.next() {
            Some(x) => x,
            None => unimplemented!(),
        };

        let id = String::from(match path_terms.next() {
            Some(x) => x,
            None => "undefined",
        });

        let response_status_line;
        let response_body;

        if id == "undefined" {
            response_status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
            response_body = "404";
        }
        else {
            self.job_callback(id)?;

            if method == "GET" {
                response_status_line = "HTTP/1.1 200 OK\r\n\r\n";
                response_body = "OK";
            } else {
                response_status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
                response_body = "404";
            }
        }

        let response = format!("{}{}", response_status_line, response_body);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

        Ok(())
    }

    pub fn job_callback (&self, job_id: String) -> Result<(), ServiceError>{
        let job = self.get_job(job_id)?;

        match job.result {
            None => {Err(ServiceError::from("This job has no result"))}
            Some(result) => {
                // TODO decide what to do with the job result
                println!("Job result: {}", result);
                Ok(())
            }
        }
    }
}

