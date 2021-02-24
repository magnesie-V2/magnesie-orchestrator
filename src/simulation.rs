pub static mut CURRENT_TIME_IN_SECONDS: i32 = 0;
pub static mut SIMULATION_STARTED: bool = false;

pub static SHORT_JOB_PHOTOS_COUNT: i32 = 10;
pub static LONG_JOB_PHOTOS_COUNT: i32 = 30;
pub static SIMULATION_1_SHORTS_COUNT: i32 = 60;
pub static SIMULATION_2_LONGS_COUNT: i32 = 20;
pub static SIMULATION_3_SHORTS_COUNT: i32 = 30;
pub static SIMULATION_3_LONGS_COUNT: i32 = 10;

pub static TIME_PER_PHOTO: i32 = 60;
pub static ITERATION_DURATION:i32 = 600;
pub static TOTAL_DURATION:i32 = 86400;

pub fn progress(){
    unsafe {
        let s = crate::simulation::PHOTOGRAMMETRY_SERVICE.lock();

        if s.is_err() {
            panic!("The photogrammetry mock is stuck");
        }

        s.unwrap().progress();
        CURRENT_TIME_IN_SECONDS += ITERATION_DURATION;
    }
}

pub fn should_end() -> bool{
    unsafe{
        CURRENT_TIME_IN_SECONDS >= TOTAL_DURATION
    }
}

pub fn get_energy_used() -> f32 {
    let s = PHOTOGRAMMETRY_SERVICE.lock();
    if s.is_err(){
        panic!("Could not get current energy usage");
    }
    s.unwrap().get_currently_used_energy()
}

pub fn get_energy_produced() -> f32 {
    unsafe{
        let t = (CURRENT_TIME_IN_SECONDS as f32) / (3600 as f32);

        let a = 0.3465; // max energy
        let b = 12f32; // offset
        let c = 2f32; //

        let energy = a * ( - ( (t as f32) - b ).powi(2)
            /
            ( 2f32 * c.powi(2) )
        ).exp();

        if energy <= 0.0001 {
            0f32
        } else {
            energy
        }
    }
}

pub fn log_energy_headers(){
    println!("datetime;energy produced;energy used");
}

pub fn log_energy(){
    unsafe {
        let time = CURRENT_TIME_IN_SECONDS;
        let energy_produced = get_energy_produced();
        let energy_used = get_energy_used();

        use chrono::{NaiveDateTime, Local};
        let h = time.wrapping_div(3600);
        let m  = (time - h*3600).wrapping_div(60);
        let s = time - h*3600 - m*60;

        let mut h = h.to_string() + "";
        let mut m  = m.to_string() + "";
        let mut s = s.to_string() + "";

        if h.len() < 2 {
            h = "0".to_string() + &*h
        }
        if m.len() < 2 {
            m = "0".to_string() + &*m
        }
        if s.len() < 2 {
            s = "0".to_string() + &*s
        }

        println!("{}T{}:{}:{};{};{}", Local::now().format("%Y-%m-%d"), h, m, s, energy_produced, energy_used);
    }
}

use lazy_static::lazy_static;
use crate::services::PhotogrammetryJob;
use std::sync::Mutex;
lazy_static!{
    pub static ref PHOTOGRAMMETRY_SERVICE: Mutex<PhotogrammetryMock> = Mutex::new(PhotogrammetryMock::new());
}

pub struct PhotogrammetryJobMock{
    pub id: String,
    pub status: String,
    pub photos: Vec<String>,
    pub callback: String,
    pub lifetime_in_seconds: i32
}

pub struct PhotogrammetryMock {
    pub jobs: Vec<PhotogrammetryJobMock>
}

impl PhotogrammetryMock{
    pub fn new() -> PhotogrammetryMock{
        PhotogrammetryMock{
            jobs: Vec::new()
        }
    }

    pub fn create_job(&mut self, images_urls: &[String], callback_url: &str) -> String {
        let id = uuid::Uuid::new_v4().to_string();

        self.jobs.push(PhotogrammetryJobMock{
            id: id.clone(),
            status: "Running".to_string(),
            photos: images_urls.to_vec().iter().map(|p| p.to_string()).collect(),
            callback: callback_url.to_string(),
            lifetime_in_seconds: 0
        });

        id.clone()
    }

    pub fn get_job(&self, id: &str) -> Option<PhotogrammetryJob> {
        for job in self.jobs.iter(){
            if job.id == id {
                return Some(PhotogrammetryJob{
                    id: Some(job.id.clone()),
                    status: Some(job.status.clone())
                });
            }
        }
        None
    }

    pub fn progress(&mut self){
        for job in self.jobs.iter_mut(){
            job.lifetime_in_seconds += ITERATION_DURATION;
        }

        self.jobs.retain(|job| {
            let photos_count = job.photos.len();
            if job.lifetime_in_seconds >= (photos_count as i32) * TIME_PER_PHOTO {
                false
            } else {
                true
            }
        });
    }

    pub fn get_currently_used_energy(&self) -> f32{
        let mut total_photos = 0;
        for job in &self.jobs{
            total_photos += job.photos.len();
        }

        return (total_photos as f32) * crate::ENERGY_COST_PER_COMPLEXITY_UNIT;
    }
}

