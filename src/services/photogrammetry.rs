use std::sync::{Arc, RwLock};

use reqwest::blocking::Client;
use serde::Deserialize;
use serde::Serialize;

use super::{Service, ServicesKeeper, ServiceError};

#[derive(Deserialize, Debug)]
/// Represents a job created by the PhotogrammetryService
pub struct PhotogrammetryJob {
    pub id: Option<String>,
    pub status: Option<String>,
}

/// Represents a request body to start a job in the PhotogrammetryService
#[derive(Serialize, Debug)]
struct PhotogrammetryJobRequestBody{
    pub photos: Vec<String>,
    pub callback: String
}

/// HTTP client for the photogrammetry microservice
pub struct PhotogrammetryService {
    services_keeper: Arc<RwLock<ServicesKeeper>>,
    client: Client, // it's best to create a client and reuse it for request pooling
}

impl PhotogrammetryService {
    /// Creates a PhotogrammetryService struct
    pub fn new(services_keeper: Arc<RwLock<ServicesKeeper>>) -> Result<PhotogrammetryService, ServiceError> {
        Ok(PhotogrammetryService {
            services_keeper,
            client: reqwest::blocking::Client::new()
        })
    }

    /// Sends pictures urls to the photogrammetry microservice and returns the id of the created job
    pub fn create_job(&self, images_urls: &[String], callback_url: &str) -> Result<String, ServiceError> {
        let s = crate::simulation::PHOTOGRAMMETRY_SERVICE.lock();

        if s.is_err() {
            return Err(ServiceError::from("simulated job creation"));
        }

        Ok(s.unwrap().create_job(images_urls, callback_url))
    }

    /// Retrieves information about a job based on its id
    pub fn get_job(&self, id: &str) -> Result<PhotogrammetryJob, ServiceError>{
        let s = crate::simulation::PHOTOGRAMMETRY_SERVICE.lock();

        if s.is_err() {
            return Err(ServiceError::from("simulated job get failed"));
        }

        match s.unwrap().get_job(id){
            None => Err(ServiceError::from("simulated job get failed: job does not exist")),
            Some(j) => Ok(j)
        }
    }

    /// Retrieves information about a job's result based on its id
    pub fn get_job_result_url(&self, id: &str) -> Result<String, ServiceError>{
        let access_information = self.get_access_information()?;

        let result_url = format!("http://{host}:{port}/res/{id}.tar.gz",
                                  host=access_information.get_host(),
                                  port=access_information.get_port(),
                                  id=id);

        Ok(result_url)
    }
}

impl Service for PhotogrammetryService{
    fn get_name(&self) -> String {
        "photogrammetry".to_string()
    }

    fn get_services_keeper(&self) -> Arc<RwLock<ServicesKeeper>> {
        self.services_keeper.clone()
    }
}
