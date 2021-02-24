use std::sync::{Arc, RwLock};

use reqwest::blocking::Client;
use serde::Deserialize;
use serde::Serialize;

use super::{Service, ServicesKeeper, ServiceError};

/// Represents a submission from the ImageStorageService
#[derive(Deserialize, Debug)]
pub struct Submission {
    pub id: i32,
    pub photos: Vec<String>,
    pub submission_date: String,
}

/// Represents a request body to edit a submission in the ImageStorageService
#[derive(Serialize, Debug)]
struct SubmissionUpdateRequestBody {
    pub id: i32,
    pub status: String,
}

/// HTTP client to the image storage microservice
pub struct ImageStorageService {
    /// Keeps track of all services access information, necessary to create http requests
    services_keeper: Arc<RwLock<ServicesKeeper>>,
    client: Client, // it's best to create a client and reuse it for request pooling
}


impl ImageStorageService {
    /// Creates a ImageStorageService struct
    pub fn new(services_keeper: Arc<RwLock<ServicesKeeper>>) -> Result<ImageStorageService, ServiceError>{
        Ok(ImageStorageService {
            services_keeper,
            client: reqwest::blocking::Client::new()
        })
    }

    /// Returns new submissions currently stored in the ImageStorageService
    ///
    /// Mock: Generates submissions based on simulation constants
    pub fn get_new_submissions(&self) -> Result<Vec<Submission>, ServiceError> {
        use rand::thread_rng;
        use rand::seq::SliceRandom;
        use std::{env, thread, time};
        use crate::simulation::*;
        unsafe {
            if crate::SIMULATION_STARTED {
                return Ok(Vec::new());
            } else {
                crate::SIMULATION_STARTED = true;
                let args: Vec<String> = env::args().collect();
                if args.len() > 1{
                    let arg1 = &args[1];
                    let simulation_id: i32 = arg1.parse().unwrap();

                    let mut short_job_photos = Vec::new();
                    for i in 1..SHORT_JOB_PHOTOS_COUNT+1{
                        short_job_photos.push(format!("{}.jpeg", i));
                    }

                    let mut long_job_photos = Vec::new();
                    for i in 1..LONG_JOB_PHOTOS_COUNT+1{
                        long_job_photos.push(format!("{}.jpeg", i));
                    }

                    if simulation_id == 1 { // 60 shorts jobs
                        let mut submissions = Vec::new();
                        for i in 1..SIMULATION_1_SHORTS_COUNT+1{
                            submissions.push(Submission{
                                id: i,
                                photos: short_job_photos.clone(),
                                submission_date: "".to_string()
                            });
                        }

                        return Ok(submissions);
                    } else if simulation_id == 2 { // 20 long jobs
                        let mut submissions = Vec::new();
                        for i in 1..SIMULATION_2_LONGS_COUNT+1{
                            submissions.push(Submission{
                                id: i,
                                photos: long_job_photos.clone(),
                                submission_date: "".to_string()
                            });
                        }

                        return Ok(submissions);
                    } else if simulation_id == 3 { // 10 longs jobs and 30 short jobs
                        let mut submissions = Vec::new();

                        for i in 1..SIMULATION_3_SHORTS_COUNT+1{
                            submissions.push(Submission{
                                id: i,
                                photos: short_job_photos.clone(),
                                submission_date: "".to_string()
                            });
                        }

                        for i in 1..SIMULATION_3_LONGS_COUNT+1{
                            submissions.push(Submission{
                                id: SIMULATION_3_SHORTS_COUNT+i,
                                photos: long_job_photos.clone(),
                                submission_date: "".to_string()
                            });
                        }

                        let mut rng = thread_rng();
                        submissions.shuffle(&mut rng);

                        return Ok(submissions);
                    } else {
                        panic!("Unknown simulation number. Valid numbers are 1, 2 and 3.");
                    }
                }

                panic!("Missing simulation number. Ex: cargo run 1");
            }
        }
    }

    /// Updates the status of a submission in the ImageStorageService
    pub fn change_submission_status(&self, id: &i32, status: &str) -> Result<(), ServiceError> {
        unimplemented!();
    }
}

impl Service for ImageStorageService {
    fn get_name(&self) -> String {
        "image storage".to_string()
    }

    fn get_services_keeper(&self) -> Arc<RwLock<ServicesKeeper>> {
        self.services_keeper.clone()
    }
}
