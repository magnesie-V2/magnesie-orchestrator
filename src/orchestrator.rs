use std::{time, thread};
use std::time::SystemTime;
use std::sync::{Arc, RwLock};
use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};

//use chrono::{DateTime};

use crate::services::{ImageStorageService, PhotogrammetryService, ServiceError, ServicesKeeper, Service, ResultStorageService};
use crate::jobs_buffer::{JobsBuffer, BufferedJob};
use crate::clusters::{ClustersManager};
use crate::{log, log_error};

const COMPLEXITY_CONSTANT: f32 = 1f32; // TODO define

pub struct Orchestrator{
    ticks_delay: u64,
    green_energy_timeout: u64,
    services_keeper: Arc<RwLock<ServicesKeeper>>,
    jobs_buffer: Arc<RwLock<JobsBuffer>>,
    clusters_manager: Arc<RwLock<ClustersManager>>,
    image_storage: Arc<ImageStorageService>,
    photogrammetry: Arc<PhotogrammetryService>,
    result_storage: Arc<ResultStorageService>
}

impl Orchestrator {
    /// @param ticks_delay delay between ticks of the orchestrator in seconds
    /// @param green_energy_timeout delay before forcing jobs processing without green energy in seconds
    pub fn new(ticks_delay: u64, green_energy_timeout: u64,
               services_keeper: Arc<RwLock<ServicesKeeper>>,jobs_buffer: Arc<RwLock<JobsBuffer>>, clusters_manager: Arc<RwLock<ClustersManager>>,
               image_storage: Arc<ImageStorageService>, photogrammetry: Arc<PhotogrammetryService>, result_storage: Arc<ResultStorageService>) -> Orchestrator{
        Orchestrator{
            ticks_delay, green_energy_timeout,
            services_keeper, jobs_buffer, clusters_manager,
            image_storage, photogrammetry, result_storage
        }
    }

    /// TODO: don't deploy with 0 jobs
    /// We have to use an associated function rather than a method here to allow using the orchestrator in different threads. Indeed, "self" could not be used in a different thread for lifetime reasons
    pub fn start(orchestrator: Arc<Orchestrator>){
        log("Orchestrator", "Starting up");
        log("Orchestrator", "Starting up web server");

        Orchestrator::start_web_server(orchestrator.clone());

        loop {
            if let Err(_) = orchestrator.add_submissions_to_buffer(){

            }

            let mut buffer = orchestrator.jobs_buffer.write().unwrap();

            if buffer.has_buffered_jobs() {
                if let Some(jobs) = buffer.get_pending_jobs(){
                    let mut jobs = jobs;

                    log("Orchestrator", "Selecting cluster");
                    if let Some(selected_cluster) = orchestrator.clusters_manager.write().unwrap().select_cluster() {

                        log("Orchestrator", "Deploying photogrammetry service");
                        if let Ok(sai) =  selected_cluster.deploy_photogrammetry_service() {
                            {
                                let mut sk = orchestrator.services_keeper.write().unwrap();
                                sk.register_service(&orchestrator.photogrammetry.get_name(), sai);
                            }

                            let energy = selected_cluster.get_green_energy_produced();
                            let jobs_to_run= orchestrator.select_jobs_to_run(&mut jobs, &energy);
                            if jobs_to_run.is_some(){
                                if let Err(_) = orchestrator.run_jobs(&mut jobs_to_run.unwrap()){

                                }
                            }

                        }
                    }
                }
            }
            drop(buffer);
            thread::sleep(time::Duration::from_secs(orchestrator.ticks_delay.clone()));
        }
    }

    /// We have to use an associated function rather than a method here to allow using the orchestrator in different threads. Indeed, "self" could not be used in a different thread for lifetime reasons
    fn start_web_server(orchestrator: Arc<Orchestrator>){
        let o_clone = orchestrator.clone();
        thread::spawn(move || -> Result<(), String>{
            match TcpListener::bind("0.0.0.0:7878"){
                Ok(callback_listener) => {
                    for stream in callback_listener.incoming(){
                        match stream{
                            Ok(s) => {
                                if let Err(_) = o_clone.handle_tcp_connection(s){

                                }
                            }
                            Err(er) => {
                                return Err(er.to_string());
                            }
                        }
                    }
                    Err("TCP Listener stopped listening prematurely".to_string())
                }
                Err(er) => {
                    Err(er.to_string())
                }
            }
        });
    }

    fn add_submissions_to_buffer(&self) -> Result<(), String>{
        log("ImageStorage", "Fetching new submissions");
        let get_new_submissions_result = self.image_storage.get_new_submissions();

        if let Err(er) = get_new_submissions_result {
            log_error(&er.to_string());
            return Err(er.to_string());
        }

        let new_submissions = get_new_submissions_result.ok().unwrap();
        log("ImageStorage", &format!("Fetched {}", new_submissions.len()));

        let mut buffer = self.jobs_buffer.write().unwrap();

        for s in new_submissions.into_iter() {
            let photos: Vec<&str> = s.photos.iter().map(|s| s as &str).collect();
            /*let submission_time = DateTime::parse_from_str(&s.submission_date, "%Y-%m-%dT%H:%M:%S%.3f%z");

            match submission_time {
                Ok(s_time) => {*/
                    let job = BufferedJob::new(&None, &photos, &s.id, SystemTime::now()/*SystemTime::from(s_time)*/);
                    if let false = buffer.submission_exists(&job) {
                        log("JobsBuffer", &format!("Adding job {}", job.to_string()));

                        if let Err(er) = buffer.add_job(job) {
                            log_error(&er.to_string());
                        }
                    }
                /*}
                Err(er) => {
                    log_error(&format!("[ERROR] Unable to parse datetime of submission {} ({}). Make sure the datetime follows the same pattern as 2021-01-17T14:32:14.184+0001", s.id, er.to_string()));
                }
            }*/
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
            } else if let Ok(time_pending) = SystemTime::now().duration_since(job.submission_date) {
                if time_pending.as_secs() >= self.green_energy_timeout {
                    total_complexity += job_complexity;
                    jobs_to_run.push(&mut (**job));
                }
            }
        }

        return Some(jobs_to_run);
    }

    fn run_jobs(&self, jobs: &mut[&mut BufferedJob]) -> Result<(), String>{
        log("Orchestrator", &format!("Sending {} job(s) to the photogrammetry service", jobs.len()));
        for job in jobs.iter_mut(){

            log("Photogrammetry", &format!("Creating a job from {} photos", job.photos.len()));
            let job_id = self.photogrammetry.create_job(&job.photos, "/photogrammetry");

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

    /// TODO Error cases, and refactor
    fn handle_tcp_connection (&self, mut stream: TcpStream) -> Result<(), String> {
        let mut buffer = [0; 1024];
        let response_status_line;
        let response_body;

        stream.read(&mut buffer).unwrap();

        let buffer_as_string;
        match std::str::from_utf8(&buffer){
            Ok(string) => {
                buffer_as_string = string.to_string();
            }
            Err(er) => {
                return Err(er.to_string());
            }
        }

        let mut request_terms = buffer_as_string.split_whitespace();

        let method = match request_terms.next() {
            Some(x) => x,
            None => unimplemented!(),
        };

        let mut path= "";
        match request_terms.next() {
            Some(x) => path = x,
            None => {}
        };

        let mut path_terms = path.split("/");
        match path_terms.next() {
            Some(_) => {},
            None => println!("Bad request: [{}] {}", method, path),
        };
        match path_terms.next() {
            Some(_) => {},
            None => println!("Bad request: [{}] {}", method, path),
        };

        let mut id = String::from(match path_terms.next() {
            Some(x) => x,
            None => "undefined",
        });

        match self.photogrammetry.get_job(&id){
            Ok(_) => {}
            Err(_) => {
                id = String::from("undefined")
            }
        };

        if id == "undefined" {
            response_status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
            response_body = "404";
        } else {
            self.photogrammetry_callback(id.as_str())?;

            if method == "GET" {
                response_status_line = "HTTP/1.1 200 OK\r\n\r\n";
                response_body = "OK";
            } else {
                response_status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
                response_body = "404";
            }
        }

        let response = format!("{}{}", response_status_line, response_body);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

        Ok(())
    }

    fn photogrammetry_callback(&self, id: &str) -> Result<(), ServiceError>{
        let result_url = self.photogrammetry.get_job_result_url(id);
        match result_url {
            Ok(result_url) => {
                let mut buffer = self.jobs_buffer.write().unwrap();
                if let Some(job) = buffer.get_job_by_id(id){
                    log("ResultStorage", &format!("Getting result of submission {}", job.submission_id));
                    if let Err(_) = self.result_storage.post_result(&job.submission_id, &result_url){

                    }

                    log("ImageStorage", &format!("Setting status of submission {} to {}", job.submission_id, "Done"));
                    if let Err(_) = self.image_storage.change_submission_status(&job.submission_id, "Done"){

                    }

                    log("JobsBuffer", &format!("Removing job {} from the buffer", id));
                    if let Err(_) = buffer.remove_job(id){

                    }
                }
                Ok(())
            }
            Err(_) => {Err(ServiceError::from("This job has no result"))}
        }
    }
}

