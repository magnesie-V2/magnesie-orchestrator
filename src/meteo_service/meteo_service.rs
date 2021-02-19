#[allow(dead_code)]
extern crate reqwest;
extern crate serde;

use serde_json::{Value};

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    error::Error, 
};

const GRID5000_SITES_FILE_PATH : &str = "ressources/grid5000_sites.txt";
const OPEN_WEATHER_MAP_CONF_FILE : &str = "config/open_weather_map.json";

pub struct MeteoClient{
    api_address : String,
    api_key : String,
}

impl MeteoClient {
    
    #[allow(dead_code)]
    pub fn new() -> MeteoClient {
        MeteoClient {
            api_address: "https://api.openweathermap.org/data/2.5/weather".to_string(),
            api_key : read_api_key_from_file().unwrap(),
        }
    }

    #[allow(dead_code)]
    pub fn get_weather_for_city(&self, city: String) -> Result<(u64, f64, u64), reqwest::Error> {

        let api_url = format!("{}?q={}&appid={}&lang=fr", self.api_address, city, self.api_key);
        let client = reqwest::blocking::Client::new();
        let res = client
            .get(api_url.as_str())
            .send()
            .expect("Failed to send request");

        // Move and borrow value of `res`
        let response_body : String = res.text().unwrap();
        let serde_resp : Value = serde_json::from_str(&response_body).unwrap();
        
        Ok((serde_resp["weather"][0]["id"].as_u64().unwrap(), serde_resp["wind"]["speed"].as_f64().unwrap(), serde_resp["sys"]["sunset"].as_u64().unwrap()))
    }
    
    pub fn get_weather_for_grid5000_sites(&self) -> Vec<(String, (u64, f64, u64))>  {
        
        let grid5000_sites : Vec<String> = self.get_sites_from_file().unwrap();
        let grid5000_sites_weatherdata : Vec<(u64, f64, u64)> = grid5000_sites.iter().map(|x| self.get_weather_for_city(x.clone()).unwrap()).collect();
        let ret = grid5000_sites.into_iter().zip(grid5000_sites_weatherdata.into_iter()).collect();

        return ret;
    }   

    fn get_sites_from_file(&self) -> io::Result<Vec<String>> {
        BufReader::new(File::open(GRID5000_SITES_FILE_PATH)?).lines().collect()
    }

}

pub fn read_api_key_from_file() -> Result<String, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(OPEN_WEATHER_MAP_CONF_FILE)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let json : serde_json::Value = serde_json::from_reader(reader)?;
    
    // Return the `User`.
    Ok(json.get("openweathermap_token").unwrap().to_string().replace("\"", ""))
}