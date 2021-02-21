use std::{thread, time};
use std::sync::{Arc, RwLock};

use crate::services::{ImageStorageService, PhotogrammetryService, ServiceError, Submission};
use crate::jobs_buffer::{JobsBuffer, BufferedJob};
use crate::clusters::{ClustersManager, Cluster};

use std::time::SystemTime;
use chrono::{DateTime, FixedOffset, ParseError, offset::Utc, NaiveDate, NaiveDateTime};

pub struct Orchestrator{
    ticks_delay: u64,
    green_energy_timeout: u64,
    jobs_buffer: Arc<RwLock<JobsBuffer>>,
    clusters_manager: Arc<ClustersManager>,
    image_storage: Arc<ImageStorageService>,
    photogrammetry: Arc<PhotogrammetryService>
}

impl Orchestrator {
    /// @param ticks_delay delay between ticks of the orchestrator in seconds
    /// @param green_energy_timeout delay before forcing jobs processing without green energy in seconds
    pub fn new(ticks_delay: u64, green_energy_timeout: u64, jobs_buffer: Arc<RwLock<JobsBuffer>>, clusters_manager: Arc<ClustersManager>,
               image_storage: Arc<ImageStorageService>,
               photogrammetry: Arc<PhotogrammetryService>) -> Orchestrator{
        Orchestrator{
            ticks_delay,
            green_energy_timeout,
            jobs_buffer,
            clusters_manager,
            image_storage,
            photogrammetry
        }
    }

    pub fn start(&self){
        loop {
            self.add_submissions_to_buffer();

            if let buffer = self.jobs_buffer.read().unwrap().has_buffered_jobs() {
                if let Some(selected_cluster) = self.clusters_manager.select_cluster() {
                    self.run_jobs(selected_cluster);
                }
            }
            thread::sleep(time::Duration::from_secs(self.ticks_delay.clone()));
        }
    }

    fn add_submissions_to_buffer(&self) -> Result<(), String>{
        println!("[ImageStorage] Fetching new submissions from the service");
        let get_new_submissions_result = self.image_storage.get_new_submissions();

        if let Err(err) = get_new_submissions_result {
            println!("[ERROR] {}", err.to_string());
            return Err(err.to_string());
        }

        let new_submissions = get_new_submissions_result.ok().unwrap();
        println!("[ImageStorage] --> OK ({} found)", new_submissions.len());

        let mut buffer = self.jobs_buffer.write().unwrap();

        for s in new_submissions.into_iter() {
            println!("[Orchestrator] Parsing datetime of submission {}", s.id);
            let photos: Vec<&str> = s.photos.iter().map(|s| s as &str).collect();
            let submission_time = DateTime::parse_from_str(&s.submission_date, "%Y-%m-%dT%H:%M:%S%.3f%z");

            match submission_time {
                Ok(s_time) => {
                    println!("[Orchestrator] --> OK");
                    let job = BufferedJob::new(&None, &photos, &s.id, SystemTime::from(s_time));
                    if let false = buffer.submission_exists(&job) {
                        println!("[JobsBuffer] Adding job {}", job.to_string());

                        if let Err(er) = buffer.add_job(job) {
                            println!("[ERROR] {}", er);
                        } else {
                            println!("[JobsBuffer] --> OK");
                        }
                    }
                }
                Err(er) => {
                    println!("[ERROR] Unable to parse datetime of submission {} ({}). Make sure the datetime follows the same pattern as 2021-01-17T14:32:14.184+0001", s.id, er.to_string());
                }
            }
        }

        Ok(())
    }

    fn run_jobs(&self, cluster: &Box<dyn Cluster>) -> Result<(), String>{
        let mut buffer = self.jobs_buffer.write().unwrap();

        if let None = buffer.get_pending_jobs() {
            return Ok(());
        }

        let pending_jobs = buffer.get_pending_jobs().unwrap();

        println!("[ORCHESTRATOR] Running {} jobs", pending_jobs.len());

        Ok(())
    }
}
