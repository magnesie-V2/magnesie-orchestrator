use std::{thread, time};
use std::sync::{Arc, RwLock};

use crate::services::{ImageStorageService, PhotogrammetryService, ServiceError, Submission};
use crate::jobs_buffer::{JobsBuffer, BufferedJob};
use std::time::SystemTime;

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
            // TODO
            /*match new_submissions {
                Ok(list) => {
                    let buffer_reader = self.jobs_buffer.read().unwrap();
                    let mut new_subs: Vec<Submission> = Vec::new();
                    for s in list.into_iter() {
                        if !buffer_reader.submission_exists(&s.id) {
                            &new_subs.push(s);
                        }
                    }
                    let mut buffer_writer = self.jobs_buffer.write().unwrap();
                    let new_subs_len = new_subs.len();
                    for s in new_subs.into_iter(){
                        buffer_writer.add_job(BufferedJob{
                            id: None,
                            photos: s.photos,
                            submission_id: s.id,
                            submission_date: SystemTime::now()
                        });
                    }

                    println!("[ORCHESTRATOR] {} submission(s) found", new_subs_len);
                }
                Err(err) => {
                    println!("{}", err.to_string());
                }
            }*/

            println!("tick");
            thread::sleep(time::Duration::from_secs(self.ticks_delay));
        }
    }
}

