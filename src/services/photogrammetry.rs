use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, RwLock};
use std::thread;
use std::io::{Read, Write};

use reqwest::blocking::Client;
use serde::Deserialize;
use serde::Serialize;

use super::{Service, ServicesKeeper, ServiceAccessInformation, ServiceError};

#[derive(Deserialize, Debug)]
pub struct PhotogrammetryJob {
    pub id: Option<String>,
    pub status: Option<String>,
}

#[derive(Serialize, Debug)]
struct PhotogrammetryJobRequestBody{
    pub photos: Vec<String>,
    pub callback: String
}

/// Rest client for the photogrammetry micro service <br /> <br />
///
///
pub struct PhotogrammetryService {
    services_keeper: Arc<RwLock<ServicesKeeper>>,
    client: Client // it's best to create a client and reuse it for request pooling
}

impl Service for PhotogrammetryService{
    fn get_name(&self) -> String {
        "photogrammetry".to_string()
    }

    fn get_services_keeper(&self) -> Arc<RwLock<ServicesKeeper>> {
        self.services_keeper.clone()
    }
}

impl PhotogrammetryService {
    pub fn new(services_keeper: Arc<RwLock<ServicesKeeper>>) -> Result<Arc<PhotogrammetryService>, ServiceError> {
        let service = Arc::new(PhotogrammetryService {
            services_keeper,
            client: reqwest::blocking::Client::new()
        });

        let service_clone = service.clone();
        thread::spawn(move || -> Result<String,ServiceError> {
            let callback_listener = TcpListener::bind("0.0.0.0:7878")?;
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
            "photo1.jpeg",
            "photo2.jpeg",
            "photo3.jpeg"
        ].to_vec();
        let photogrammetry_callback = "/photogrammetry/<id>";

        let id = self.create_job(&mock_photos, photogrammetry_callback)?;
        println!("Created job of id: {}", id);

        let job = self.get_job(id.as_str())?;
        println!("Job of id {} is currently: {}", job.id.unwrap(), job.status.unwrap());

        Ok(true) // No error thrown so the test returns true
    }

    /// Sends pictures urls to the photogrammetry webservice and returns the id of the created job
    pub fn create_job(&self, images_urls: &[&str], callback_url: &str) -> Result<String, ServiceError> {
        println!("[Photogrammetry] Creating a job from {} photos", images_urls.len());
        let access_information = self.get_access_information()?;

        let request_url = format!("http://{host}:{port}/job",
                                  host=access_information.get_host(),
                                  port=access_information.get_port());

        let body = PhotogrammetryJobRequestBody {
            photos: images_urls.to_vec().iter().map(|s| s.to_string()).collect(),
            callback: String::from(callback_url)
        };

        let request = self.client.post(&request_url).json(&body);

        let response = request.send()?;
        let response_body: PhotogrammetryJob = response.json()?;

        match response_body.id {
            None => Err(ServiceError::from("The id field wasn't found in the response body")),
            Some(id) => Ok(id)
        }
    }

    /// Retrieves information about a job based on its id
    pub fn get_job(&self, id: &str) -> Result<PhotogrammetryJob, ServiceError>{
        println!("[Photogrammetry] Getting information about job {}", id);
        let access_information = self.get_access_information()?;

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
    fn get_job_result_url(&self, id: &str) -> Result<String, ServiceError>{
        println!("[Photogrammetry] Getting job {} result url", id);
        let access_information = self.get_access_information()?;

        let result_url = format!("http://{host}:{port}/res/{id}.tar.gz",
                                  host=access_information.get_host(),
                                  port=access_information.get_port(),
                                  id=id);

        Ok(result_url)
    }

    /// TODO Error cases, and refactor
    fn handle_connection (&self, mut stream: TcpStream) -> Result<(), ServiceError> {
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

    fn job_callback (&self, job_id: &str) -> Result<(), ServiceError>{
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

#[test]
pub fn test(){
    let services_keeper = Arc::new(RwLock::new(ServicesKeeper::new()));

    let photogrammetry_access_info = ServiceAccessInformation::new(
        "8645cc99-fdca-4a6a-bf45-1eb639a54f2c.mock.pstmn.io", // 8645cc99-fdca-4a6a-bf45-1eb639a54f2c.mock.pstmn.io           172.17.0.1
        80, // 80             7979
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

    // loop{} // preventing the app to terminate to avoid having to join services threads with the main thread
}
