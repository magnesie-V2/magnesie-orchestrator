use std::sync::{Arc, RwLock};

use reqwest::blocking::Client;
use serde::Deserialize;
use serde::Serialize;

use super::{Service, ServicesKeeper, ServiceError};

/// Represents a submission from the ImageStorageService
#[derive(Deserialize, Debug)]
pub struct Submission {
    pub id: i32,
    pub name: String,
    pub photos: Vec<String>,
    pub submission_date: String,
}

/// Represents a request body to edit a submission in the ImageStorageService
#[derive(Serialize, Debug)]
struct SubmissionUpdateRequestBody {
    pub id: i32,
    pub status: String,
}

/// HTTP client to the image storage microservice
pub struct ImageStorageService {
    /// Keeps track of all services access information, necessary to create http requests
    services_keeper: Arc<RwLock<ServicesKeeper>>,
    client: Client, // it's best to create a client and reuse it for request pooling
}


impl ImageStorageService {
    /// Creates a ImageStorageService struct
    pub fn new(services_keeper: Arc<RwLock<ServicesKeeper>>) -> Result<ImageStorageService, ServiceError>{
        Ok(ImageStorageService {
            services_keeper,
            client: reqwest::blocking::Client::new()
        })
    }

    /// Returns new submissions currently stored in the ImageStorageService
    pub fn get_new_submissions(&self) -> Result<Vec<Submission>, ServiceError> {
        let access_information = self.get_access_information()?;

        let request_url = format!("http://{host}:{port}/new_submissions",
                                  host=access_information.get_host(),
                                  port=access_information.get_port());

        let request = self.client.get(&request_url);

        let response = request.send()?;
        let response_body: Vec<Submission> = response.json()?;



        Ok(response_body)
    }

    /// Updates the status of a submission in the ImageStorageService
    pub fn change_submission_status(&self, id: &i32, status: &str) -> Result<(), ServiceError> {
        let access_information = self.get_access_information()?;

        let request_url = format!("http://{host}:{port}/change_submission_status",
                                  host=access_information.get_host(),
                                  port=access_information.get_port());

        let body = SubmissionUpdateRequestBody {
            id: id.clone(),
            status: status.to_string()
        };

        let request = self.client.post(&request_url).json(&body);
        let response = request.send()?;

        if response.status().is_success() {
            return Ok(());
        }

        Err(ServiceError::from(response.status().to_string()))
    }
}

impl Service for ImageStorageService {
    fn get_name(&self) -> String {
        "image storage".to_string()
    }

    fn get_services_keeper(&self) -> Arc<RwLock<ServicesKeeper>> {
        self.services_keeper.clone()
    }
}
