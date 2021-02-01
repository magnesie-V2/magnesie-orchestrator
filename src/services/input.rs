use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, RwLock};

use std::time::SystemTime;
use chrono::offset::Utc;
use chrono::DateTime;

use reqwest::blocking::Client;
use serde::Deserialize;
use serde::Serialize;

use super::{Service, ServicesKeeper, ServiceAccessInformation, ServiceError};

#[derive(Deserialize, Debug)]
pub struct InputJob {
    pub id: u32,
    pub photos: Vec<String>,
    pub submission_date: SystemTime
}

pub struct InputService{
    services_keeper: Arc<RwLock<ServicesKeeper>>,
    client: Client // it's best to create a client and reuse it for request pooling
}

impl InputService {
    pub fn new(services_keeper: Arc<RwLock<ServicesKeeper>>) -> Result<Arc<InputService>, ServiceError>{
        let service = Arc::new(InputService {
            services_keeper,
            client: reqwest::blocking::Client::new()
        });

        Ok(service)
    }

    pub fn get_new_submissions(&self) -> Result<Vec<InputJob>, ServiceError> {
        let access_information = self.get_access_information()?;

        let request_url = format!("http://{host}:{port}/new_submissions",
                                  host=access_information.get_host(),
                                  port=access_information.get_port());

        let request = self.client.get(&request_url);

        let response = request.send()?;
        let response_body: Vec<InputJob> = response.json()?;

        Ok(response_body)
    }
}

impl Service for InputService{
    fn get_name(&self) -> String {
        "input".to_string()
    }

    fn get_services_keeper(&self) -> Arc<RwLock<ServicesKeeper>> {
        self.services_keeper.clone()
    }
}

#[test]
pub fn test(){
    let services_keeper = Arc::new(RwLock::new(ServicesKeeper::new()));

    let input_access_info = ServiceAccessInformation::new(
        "645cc99-fdca-4a6a-bf45-1eb639a54f2c.mock.pstmn.io",
        80,
        "",
        "",
    );

    match InputService::new(services_keeper.clone()) {
        Ok(service) => {
            println!("Test: using the input service without any cluster selection and service deployment");
            match service.get_new_submissions() {
                Ok(_) => {}
                Err(error) => println!("{}", error)
            }

            println!("Setting the input service information (mock of cluster selection)");
            services_keeper.write().unwrap().register_service("input", input_access_info);

            println!("Test: using the input service");
            match service.get_new_submissions() {
                Ok(jobs) => {
                    println!("Found {} job(s)!", jobs.len());
                }
                Err(error) => println!("{}", error)
            }

        },
        Err(_) => unimplemented!(),
    };
}
