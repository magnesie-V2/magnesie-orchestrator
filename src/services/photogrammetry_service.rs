use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, RwLock};
use std::thread;
use std::io::{Read, Write};

use reqwest::blocking::Client;
use serde::Deserialize;
use serde::Serialize;

use super::service_error::ServiceError;
use super::services_keeper::ServicesKeeper;
// use crate::services::service_access_information::ServiceAccessInformation;

#[derive(Deserialize, Debug)]
pub struct PhotogrammetryJob {
    pub id: Option<String>,
    pub status: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct PhotogrammetryJobRequestBody{
    pub photos: Vec<String>,
    pub callback: String
}

/// Rest client for the photogrammetry service
pub struct PhotogrammetryService {
    services_keeper: Arc<RwLock<ServicesKeeper>>,
    client: Client // it's best to create a client and reuse it for request pooling
}

#[allow(dead_code)]
impl PhotogrammetryService {
    pub fn new(services_keeper: Arc<RwLock<ServicesKeeper>>) -> Result<Arc<PhotogrammetryService>, ServiceError> {
        let callback_listener = TcpListener::bind("0.0.0.0:7878")?;

        let service = Arc::new(PhotogrammetryService {
            services_keeper,
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
    pub fn test(&self) -> Result<bool, ServiceError>{
        let mock_photos = [
            String::from("photo1.jpeg"),
            String::from("photo2.jpeg"),
            String::from("photo3.jpeg")
        ].to_vec();
        let photogrammetry_callback = String::from("/photogrammetry/<id>"); // TODO get ip or orchestrator

        let id = self.create_job(mock_photos, photogrammetry_callback)?;
        println!("Created job of id: {}", id);

        let job = self.get_job(id.as_str())?;
        println!("Job of id {} is currently: {}", job.id.unwrap(), job.status.unwrap());

        Ok(true) // No error thrown so the test returns true
    }

    /// Sends pictures urls to the photogrammetry webservice and returns the id of the created job
    pub fn create_job(&self, pictures_urls: Vec<String>, callback_url: String) -> Result<String, ServiceError> {
        let services_keeper = self.services_keeper.read().unwrap();
        let access_information;

        match services_keeper.get_service("photogrammetry"){
            None => {
                return Err(ServiceError::from("No photogrammetry service available"));
            },
            Some(ai) => {
                access_information = ai;
            }
        }

        let request_url = format!("http://{host}:{port}/job",
                                  host=access_information.get_host(),
                                  port=access_information.get_port());

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
    pub fn get_job(&self, id: &str) -> Result<PhotogrammetryJob, ServiceError>{
        let services_keeper = self.services_keeper.read().unwrap();
        let access_information;

        match services_keeper.get_service("photogrammetry"){
            None => {
                return Err(ServiceError::from("No photogrammetry service available"));
            },
            Some(ai) => {
                access_information = ai;
            }
        }

        let request_url = format!("http://{host}:{port}/job/{id}",
                                  host=access_information.get_host(),
                                  port=access_information.get_port(),
                                  id=id);

        let request = self.client.get(&request_url);

        let response = request.send()?;
        let mut response_body: PhotogrammetryJob = response.json()?;

        match response_body.id {
            None => response_body.id = Some(String::from(id)),
            _ => {}
        }

        Ok(response_body)
    }

    /// Retrieves information about a job's result based on its id
    pub fn get_job_result_url(&self, id: &str) -> Result<String, ServiceError>{
        let services_keeper = self.services_keeper.read().unwrap();
        let access_information;

        match services_keeper.get_service("photogrammetry"){
            None => {
                return Err(ServiceError::from("No photogrammetry service available"));
            },
            Some(ai) => {
                access_information = ai;
            }
        }

        let result_url = format!("http://{host}:{port}/res/{id}.tar.gz",
                                  host=access_information.get_host(),
                                  port=access_information.get_port(),
                                  id=id);

        Ok(result_url)
    }

    /// TODO Error cases
    pub fn handle_connection (&self, mut stream: TcpStream) -> Result<(), ServiceError> {
        let mut buffer = [0; 1024];
        let response_status_line;
        let response_body;

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
            Some(_) => {},
            None => println!("Bad request: [{}] {}", method, path),
        };
        match path_terms.next() {
            Some(_) => {},
            None => println!("Bad request: [{}] {}", method, path),
        };

        let mut id = String::from(match path_terms.next() {
            Some(x) => x,
            None => "undefined",
        });

        match self.get_job(&id){
            Ok(_) => {}
            Err(_) => {
                id = String::from("undefined")
            }
        };

        if id == "undefined" {
            response_status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
            response_body = "404";
        }
        else {
            self.job_callback(id.as_str())?;

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

    pub fn job_callback (&self, job_id: &str) -> Result<(), ServiceError>{
        let result_url = self.get_job_result_url(job_id);
        match result_url {
            Ok(result_url) => {
                // TODO decide what to do with the job result's url
                println!("Job result url: {}", result_url);
                Ok(())
            }
            Err(_) => {Err(ServiceError::from("This job has no result"))}
        }
    }
}

