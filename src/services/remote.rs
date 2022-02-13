use std::collections::HashMap;

use crate::{log};

/// Keeps a map of the micro services access information <br />
pub struct ServicesKeeper {
    services: HashMap<String, ServiceAccessInformation>,
}

impl ServicesKeeper {
    /// Creates a ServiceKeeper struct
    pub fn new() -> ServicesKeeper{
        let services = HashMap::new();

        ServicesKeeper {
            services
        }
    }

    /// Adds a service's access information to the map at a specific key
    pub fn register_service(&mut self, service_key: &str, service_access_information: ServiceAccessInformation) {
        log("Remote", &format!("Add service {} ({}, {})", service_key, service_access_information.host, service_access_information.port));
        self.services.insert(String::from(service_key), service_access_information);
    }

    /// Returns a service's access information based on a key
    pub fn get_service(&self, service_key: &str) -> Option<&ServiceAccessInformation>{
        self.services.get(service_key)
    }
}

/// Contains access information of a webservice
pub struct ServiceAccessInformation {
    host: String,
    port: u16,
    username: String,
    password: String,
}

impl ServiceAccessInformation {
    /// Creates a ServiceAccessInformation struct
    pub fn new (host: &str, port: u16, username: &str, password: &str) -> ServiceAccessInformation{
        ServiceAccessInformation {
            host: String::from(host),
            port,
            username: String::from(username),
            password: String::from(password),
        }
    }

    /// Returns the host of the webservice
    pub fn get_host(&self) -> &str { &self.host }
    /// Returns the port that the webservice listens to
    pub fn get_port(&self) -> &u16 { &self.port }
    /// Returns the username needed to send requests to the webservice
    pub fn get_username(&self) -> &str { &self.username }
    /// Returns the password needed to send requests to the webservice
    pub fn get_password(&self) -> &str { &self.password }
}

