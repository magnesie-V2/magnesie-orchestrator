use std::sync::{Arc, RwLock};

use reqwest::blocking::Client;
use serde::Serialize;

use super::{Service, ServicesKeeper, ServiceError};

/// Request body of a result to send to the ResultStorageService
#[derive(Serialize, Debug)]
struct ResultRequestBody {
    pub submission_id: i32,
    pub name: String,
    pub result_url: String,
}

/// HTTP client for the ResultStorageService
pub struct ResultStorageService {
    services_keeper: Arc<RwLock<ServicesKeeper>>,
    client: Client, // it's best to create a client and reuse it for request pooling
}

impl ResultStorageService {
    /// Creates a ResultStorageService struct
    pub fn new(services_keeper: Arc<RwLock<ServicesKeeper>>) -> Result<ResultStorageService, ServiceError>{
        Ok(ResultStorageService {
            services_keeper,
            client: reqwest::blocking::Client::new()
        })
    }

    /// Sends a result url to the result storage service
    pub fn post_result(&self, id: &i32, name: &str, result_url: &str) -> Result<(), ServiceError> {
        let access_information = self.get_access_information()?;

        let request_url = format!("http://{host}:{port}/result",
                                  host=access_information.get_host(),
                                  port=access_information.get_port());

        let body = ResultRequestBody {
            submission_id: id.clone(),
            name: name.to_string(),
            result_url: result_url.to_string()
        };

        let request = self.client.post(&request_url).json(&body);
        let response = request.send()?;

        if response.status().is_success() {
            return Ok(());
        }

        Err(ServiceError::from(response.status().to_string()))
    }
}

impl Service for ResultStorageService {
    fn get_name(&self) -> String {
        "result storage".to_string()
    }

    fn get_services_keeper(&self) -> Arc<RwLock<ServicesKeeper>> {
        self.services_keeper.clone()
    }
}
