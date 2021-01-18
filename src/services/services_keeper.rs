use std::collections::HashMap;

use super::service_access_information::ServiceAccessInformation;
use super::service_error::ServiceError;

pub struct ServicesKeeper {
    services: HashMap<&'static str, ServiceAccessInformation>,
}

#[allow(dead_code)]
impl ServicesKeeper {
    pub fn new() -> ServicesKeeper{
        let services = HashMap::new();

        ServicesKeeper {
            services
        }
    }

    pub fn register_service(&mut self, service_key: &'static str, service_access_information: ServiceAccessInformation) {
        self.services.insert(service_key, service_access_information);
    }

    pub fn get_service(&self, service_key: &str) -> Result<&ServiceAccessInformation, ServiceError>{
        Ok(match self.services.get(service_key) {
            Some(x) => x,
            None => unimplemented!(),
        })
    }
}

