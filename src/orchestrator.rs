use std::{thread, time};
use std::sync::{Arc, RwLock};

use crate::services::{ImageStorageService, PhotogrammetryService, ServiceError, Submission};
use crate::jobs_buffer::{JobsBuffer, BufferedJob};

use std::time::SystemTime;
use chrono::{DateTime, FixedOffset, ParseError, offset::Utc, NaiveDate, NaiveDateTime};

pub struct Orchestrator{
    ticks_delay: u64,
    green_energy_timeout: u64,
    jobs_buffer: Arc<RwLock<JobsBuffer>>,
    image_storage: Arc<ImageStorageService>,
    photogrammetry: Arc<PhotogrammetryService>
}

impl Orchestrator {
    /// @param ticks_delay delay between ticks of the orchestrator in seconds
    /// @param green_energy_timeout delay before forcing jobs processing without green energy in seconds
    pub fn new(ticks_delay: u64, green_energy_timeout: u64, jobs_buffer: Arc<RwLock<JobsBuffer>>,
               image_storage: Arc<ImageStorageService>,
               photogrammetry: Arc<PhotogrammetryService>) -> Orchestrator{
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
            // println!("[ORCHESTRATOR] Tick");
            if let Err(er) = self.add_submissions_to_buffer() { println!("{}", er.to_string()) }

            if let buffer = self.jobs_buffer.read().unwrap().has_buffered_jobs() {
                drop(buffer);
                // println!("Green energy workflow");
            }
            thread::sleep(time::Duration::from_secs(self.ticks_delay.clone()));
        }
    }

    fn add_submissions_to_buffer(&self) -> Result<(), ServiceError>{
        let new_submissions = self.image_storage.get_new_submissions()?;

        let mut buffer = self.jobs_buffer.write().unwrap();

        for s in new_submissions.into_iter() {
            let photos: Vec<&str> = s.photos.iter().map(|s| s as &str).collect();
            let submission_time = DateTime::parse_from_str(&s.submission_date, "%Y-%m-%dT%H:%M:%S%.3f%z");

            match submission_time {
                Ok(s_time) => {
                    let job = BufferedJob::new(&None, &photos, &s.id, SystemTime::from(s_time));
                    if let false = buffer.submission_exists(&job) {
                        buffer.add_job(job);
                    }
                }
                Err(er) => {
                    println!("[ORCHESTRATOR] Error: unable to parse datetime of submission {} ({}). Make sure the datetime follows the same pattern as 2021-01-17T14:32:14.184+0001", s.id, er.to_string());
                }
            }
        }

        Ok(())
    }
}