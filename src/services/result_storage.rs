use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, RwLock};

use std::time::SystemTime;
use chrono::offset::Utc;
use chrono::DateTime;

use reqwest::blocking::Client;
use serde::Deserialize;
use serde::Serialize;

use super::{Service, ServicesKeeper, ServiceAccessInformation, ServiceError};

#[derive(Serialize, Debug)]
struct ResultRequestBody {
    pub submission_id: i32,
    pub result_url: String,
}

pub struct ResultStorageService {
    services_keeper: Arc<RwLock<ServicesKeeper>>,
    client: Client, // it's best to create a client and reuse it for request pooling
}

impl ResultStorageService {
    pub fn new(services_keeper: Arc<RwLock<ServicesKeeper>>) -> Result<Arc<ResultStorageService>, ServiceError>{
        let service = Arc::new(ResultStorageService {
            services_keeper,
            client: reqwest::blocking::Client::new()
        });

        Ok(service)
    }

    pub fn post_result(&self, id: &i32, result_url: &str) -> Result<(), ServiceError> {
        let access_information = self.get_access_information()?;

        let request_url = format!("http://{host}:{port}/result",
                                  host=access_information.get_host(),
                                  port=access_information.get_port());

        let body = ResultRequestBody {
            submission_id: id.clone(),
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

#[test]
pub fn test(){
    let services_keeper = Arc::new(RwLock::new(ServicesKeeper::new()));

    let input_access_info = ServiceAccessInformation::new(
        "localhost",
        7881,
        "",
        "",
    );

    match ResultStorageService::new(services_keeper.clone()) {
        Ok(service) => {
            println!("Setting the result storage service information");
            services_keeper.write().unwrap().register_service("result storage", input_access_info);

            println!("Test: using the image storage service");
            match service.post_result(&1, "http://localhost:7879/eazra-azeazr-azdaz-dbe1.tar.gz") {
                Ok(jobs) => {
                    println!("Result posted");
                }
                Err(error) => println!("{}", error)
            }

        },
        Err(_) => unimplemented!(),
    };
}