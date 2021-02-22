use std::{thread, time};
use std::sync::{Arc, RwLock};

use crate::services::{ImageStorageService, PhotogrammetryService, ServiceError, Submission, ServicesKeeper, Service};
use crate::jobs_buffer::{JobsBuffer, BufferedJob};
use crate::clusters::{ClustersManager, Cluster};

use std::time::SystemTime;
use chrono::{DateTime, FixedOffset, ParseError, offset::Utc, NaiveDate, NaiveDateTime};

use crate::{log, log_error};

const COMPLEXITY_CONSTANT: f32 = 1f32; // TODO define

pub struct Orchestrator{
    ticks_delay: u64,
    green_energy_timeout: u64,
    services_keeper: Arc<RwLock<ServicesKeeper>>,
    jobs_buffer: Arc<RwLock<JobsBuffer>>,
    clusters_manager: Arc<RwLock<ClustersManager>>,
    image_storage: Arc<ImageStorageService>,
    photogrammetry: Arc<PhotogrammetryService>
}

impl Orchestrator {
    /// @param ticks_delay delay between ticks of the orchestrator in seconds
    /// @param green_energy_timeout delay before forcing jobs processing without green energy in seconds
    pub fn new(ticks_delay: u64,
               green_energy_timeout: u64,
               services_keeper: Arc<RwLock<ServicesKeeper>>,jobs_buffer:
               Arc<RwLock<JobsBuffer>>, clusters_manager:
               Arc<RwLock<ClustersManager>>,
               image_storage: Arc<ImageStorageService>, photogrammetry: Arc<PhotogrammetryService>) -> Orchestrator{
        Orchestrator{
            ticks_delay,
            green_energy_timeout,
            services_keeper,
            jobs_buffer,
            clusters_manager,
            image_storage,
            photogrammetry
        }
    }

    pub fn start(&self){
        log("Orchestrator", "Starting");
        //loop {
            self.add_submissions_to_buffer();

            let mut buffer = self.jobs_buffer.write().unwrap();

            if buffer.has_buffered_jobs() {
                if let Some(jobs) = buffer.get_pending_jobs(){
                    let mut jobs = jobs;

                    log("Orchestrator", "Selecting cluster");
                    if let Some(selected_cluster) = self.clusters_manager.read().unwrap().select_cluster() {

                        log("Orchestrator", "Deploying photogrammetry service");
                        if let Ok(sai) = selected_cluster.deploy_photogrammetry_service() {
                            {
                                let mut sk = self.services_keeper.write().unwrap();
                                sk.register_service(&self.photogrammetry.get_name(), sai);
                            }

                            let energy = selected_cluster.get_green_energy_produced();
                            let mut jobs_to_run= self.select_jobs_to_run(&mut jobs, &energy);
                            if jobs_to_run.is_some(){
                                self.run_jobs(&mut jobs_to_run.unwrap());
                            }

                        }
                    }
                }
            }
            thread::sleep(time::Duration::from_secs(self.ticks_delay.clone()));
        //}
        loop{}
    }

    fn add_submissions_to_buffer(&self) -> Result<(), String>{
        log("ImageStorage", "Fetching new submissions from the service");
        let get_new_submissions_result = self.image_storage.get_new_submissions();

        if let Err(er) = get_new_submissions_result {
            log_error(&er.to_string());
            return Err(er.to_string());
        }

        let new_submissions = get_new_submissions_result.ok().unwrap();

        let mut buffer = self.jobs_buffer.write().unwrap();

        for s in new_submissions.into_iter() {
            let photos: Vec<&str> = s.photos.iter().map(|s| s as &str).collect();
            let submission_time = DateTime::parse_from_str(&s.submission_date, "%Y-%m-%dT%H:%M:%S%.3f%z");

            match submission_time {
                Ok(s_time) => {
                    let job = BufferedJob::new(&None, &photos, &s.id, SystemTime::from(s_time));
                    if let false = buffer.submission_exists(&job) {
                        log("JobsBuffer", &format!("Adding job {}", job.to_string()));

                        if let Err(er) = buffer.add_job(job) {
                            log_error(&er.to_string());
                        }
                    }
                }
                Err(er) => {
                    log_error(&format!("[ERROR] Unable to parse datetime of submission {} ({}). Make sure the datetime follows the same pattern as 2021-01-17T14:32:14.184+0001", s.id, er.to_string()));
                }
            }
        }

        Ok(())
    }

    // Todo : choose jobs based on complexity (job.get_complexity()) and available energy
    fn select_jobs_to_run<'a>(&self, jobs: &'a mut[&'a mut BufferedJob], available_energy: &'a Option<f32>) -> Option<Vec<&'a mut BufferedJob>> {
        log("Orchestrator", "Selecting jobs to run");
        let mut jobs_to_run = Vec::new();
        let mut total_complexity = 0f32;


        for job in jobs.iter_mut() {
            let job_complexity = job.get_complexity();

            if available_energy.is_some() && total_complexity + job_complexity < available_energy.unwrap() * COMPLEXITY_CONSTANT {
                total_complexity += job_complexity;
                jobs_to_run.push(&mut(**job));
            } else if true { // TODO: timeout handling
                total_complexity += job_complexity;
                jobs_to_run.push(&mut (**job));
            }
        }

        return Some(jobs_to_run);
    }

    fn run_jobs(&self, jobs: &mut[&mut BufferedJob]) -> Result<(), String>{
        log("Orchestrator", &format!("[Orchestrator] Sending {} job(s) to the photogrammetry service", jobs.len()));
        for job in jobs.iter_mut(){

            log("Photogrammetry", &format!("Creating a job from {} photos", job.photos.len()));
            let job_id = self.photogrammetry.create_job(&job.photos, "/photogrammetry");
            let submission_id = job.submission_id;

            match job_id{
                Ok(id) => {
                    (**job).id = Some(id.clone());
                    log("Photogrammetry", &format!("Created job {} from submission {}", id, job.submission_id));
                },
                Err(er) => {
                    log_error(&er.to_string());
                }
            }
        }

        Ok(())
    }
}

