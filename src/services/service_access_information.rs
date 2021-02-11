/// Access information to a web service, using basic auth
pub struct ServiceAccessInformation {
    host: String,
    port: u16,
    username: String,
    password: String,
}

impl ServiceAccessInformation {
    #[allow(dead_code)]
    pub fn new (host: String, port: u16, username: String, password: String) -> ServiceAccessInformation{
        ServiceAccessInformation {
            host,
            port,
            username,
            password,
        }
    }

    #[allow(dead_code)]
    pub fn get_host(&self) -> &String { &self.host }
    #[allow(dead_code)]
    pub fn get_port(&self) -> &u16 { &self.port }
    #[allow(dead_code)]
    pub fn get_username(&self) -> &String { &self.username }
    #[allow(dead_code)]
    pub fn get_password(&self) -> &String { &self.password }
}

