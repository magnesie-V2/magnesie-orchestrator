use std::{thread, time};
use std::sync::{Arc, RwLock};

use crate::services::{ImageStorageService, PhotogrammetryService, ServiceError, Submission};
use crate::jobs_buffer::JobsBuffer;

pub struct Orchestrator{
    pub ticks_delay: u64,
    pub green_energy_timeout: u64,
    jobs_buffer: Arc<RwLock<JobsBuffer>>,
    image_storage: Arc<ImageStorageService>,
    photogrammetry: Arc<PhotogrammetryService>
}

impl Orchestrator {
    /// @param ticks_delay delay between ticks of the orchestrator in seconds
    /// @param green_energy_timeout delay before forcing jobs processing without green energy in seconds
    pub fn new(ticks_delay: u64, green_energy_timeout: u64, jobs_buffer: Arc<RwLock<JobsBuffer>>, image_storage: Arc<ImageStorageService>, photogrammetry: Arc<PhotogrammetryService>) -> Orchestrator{
        Orchestrator{
            ticks_delay,
            green_energy_timeout,
            jobs_buffer,
            image_storage,
            photogrammetry
        }
    }

    pub fn start(&self){
        loop {
            let new_submissions = self.image_storage.get_new_submissions();
            match new_submissions {
                Ok(list) => {
                    println!("[ORCHESTRATOR] {} submission(s) found", list.len());
                }
                Err(err) => {
                    println!("[ORCHESTRATOR] Could not get new submissions : {}", err.to_string());
                }
            }

            thread::sleep(time::Duration::from_secs(self.ticks_delay));
        }
    }
}

