#[allow(dead_code)]
extern crate reqwest;
extern crate serde;

use serde_json::{Value};

pub struct MeteoClient{
    api_address : String,
    api_key : String
}

impl MeteoClient {
    
    #[allow(dead_code)]
    pub fn new(api_key: String) -> MeteoClient {
        MeteoClient {
            api_address: "https://api.openweathermap.org/data/2.5/weather".to_string(),
            api_key,
        }
    }

    #[allow(dead_code)]
    pub fn get_weather_for_city(&self, city: String) -> Result<(String, f64), reqwest::Error> {
        let api_url = format!("{}?q={}&appid={}&lang=fr", self.api_address, city, self.api_key);
        let client = reqwest::blocking::Client::new();
        let res = client
            .get(api_url.as_str())
            .send()
            .expect("Failed to send request");

        // Move and borrow value of `res`
        let response_body : String = res.text().unwrap();
        let serde_resp : Value = serde_json::from_str(&response_body).unwrap();
        
        Ok((serde_resp["weather"][0]["main"].to_string(), serde_resp["wind"]["speed"].as_f64().unwrap()))
    }
    
    pub fn get_weather_for_grid5000_sites(&self) -> Vec<(String, (String, f64))>  {
        
        let grid5000_sites : Vec<String> = ["grenoble", "lille", "luxembourg", "lyon", "nancy", "nantes", "rennes", "sophia"].to_vec().iter().map(|x| x.to_string()).collect();
        let grid5000_sites_weatherdata : Vec<(String, f64)> = grid5000_sites.iter().map(|x| self.get_weather_for_city(x.clone()).unwrap()).collect();
        let ret = grid5000_sites.into_iter().zip(grid5000_sites_weatherdata.into_iter()).collect();
        print!("{:?}", ret);

        return ret;
    }   
}