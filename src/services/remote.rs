use std::collections::HashMap;

/// Keeps a map of the micro services access information <br /> <br />
/// register_service() <br /> get_service()
pub struct ServicesKeeper {
    services: HashMap<String, ServiceAccessInformation>,
}

impl ServicesKeeper {
    pub fn new() -> ServicesKeeper{
        let services = HashMap::new();

        ServicesKeeper {
            services
        }
    }

    pub fn register_service(&mut self, service_key: &str, service_access_information: ServiceAccessInformation) {
        self.services.insert(String::from(service_key), service_access_information);
    }

    pub fn get_service(&self, service_key: &str) -> Option<&ServiceAccessInformation>{
        self.services.get(service_key)
    }
}

pub struct ServiceAccessInformation {
    host: String,
    port: u16,
    username: String,
    password: String,
}

impl ServiceAccessInformation {
    pub fn new (host: &str, port: u16, username: &str, password: &str) -> ServiceAccessInformation{
        ServiceAccessInformation {
            host: String::from(host),
            port,
            username: String::from(username),
            password: String::from(password),
        }
    }

    pub fn get_host(&self) -> &str { &self.host }
    pub fn get_port(&self) -> &u16 { &self.port }
    pub fn get_username(&self) -> &str { &self.username }
    pub fn get_password(&self) -> &str { &self.password }
}

