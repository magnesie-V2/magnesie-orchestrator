/// Access information of a web service, using basic auth

#[allow(dead_code)]
pub struct ServiceAccessInformation {
    host: String,
    port: u16,
    username: String,
    password: String,
}

#[allow(dead_code)]
impl ServiceAccessInformation {
    pub fn new (host: String, port: u16, username: String, password: String) -> ServiceAccessInformation{
        ServiceAccessInformation {
            host,
            port,
            username,
            password,
        }
    }

    pub fn get_host(&self) -> &String { &self.host }
    pub fn get_port(&self) -> &u16 { &self.port }
    pub fn get_username(&self) -> &String { &self.username }
    pub fn get_password(&self) -> &String { &self.password }
}

