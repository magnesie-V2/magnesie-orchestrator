use std::env;
use std::{time, thread};
use std::time::SystemTime;
use std::sync::{Arc, RwLock};
use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};

use crate::services::{ImageStorageService, PhotogrammetryService, ServiceError, ServicesKeeper, Service, ResultStorageService, ServiceAccessInformation};
use crate::jobs_buffer::{JobsBuffer, BufferedJob};
use crate::clusters::{ClustersManager, ReservationStatus, Cluster};
use crate::{log, log_error};

/// This constants needs to be set to weight the energy cost of calculations
const ENERGY_COST_PER_COMPLEXITY_UNIT: f32 = 1f32;

/// The brain of the application. Its purpose is to orchestrate all the microservices while following energetic requirements
pub struct Orchestrator{
    /// Delay in seconds between iterations of the orchestrator main loop (cf Orchestrator::start())
    ticks_delay: u64,
    /// Delay in seconds before forcing jobs processing without waiting for green energy
    green_energy_timeout: u64,
    /// Keeps information to access microservices (hostname, port, username, password)
    services_keeper: Arc<RwLock<ServicesKeeper>>,
    /// Keeps the list of ongoing submissions and jobs. Note: a job is a submission that has been sent to the photogrammetry service
    jobs_buffer: Arc<RwLock<JobsBuffer>>,
    /// Keeps the list of clusters where the photogrammetry service can be deployed in
    clusters_manager: Arc<RwLock<ClustersManager>>,
    /// Client for the images storage service, which stores the images of end users submissions
    image_storage: Arc<ImageStorageService>,
    /// Client for the photogrammetry microservice
    photogrammetry: Arc<PhotogrammetryService>,
    /// Client for the results storage microservice, which stores the result of photogrammetry calculations
    result_storage: Arc<ResultStorageService>
}

impl Orchestrator {
    /// Constructs an Orchestrator struct
    pub fn new(ticks_delay: u64, green_energy_timeout: u64,
               services_keeper: Arc<RwLock<ServicesKeeper>>,jobs_buffer: Arc<RwLock<JobsBuffer>>, clusters_manager: Arc<RwLock<ClustersManager>>,
               image_storage: Arc<ImageStorageService>, photogrammetry: Arc<PhotogrammetryService>, result_storage: Arc<ResultStorageService>) -> Orchestrator{
        Orchestrator{
            ticks_delay, green_energy_timeout,
            services_keeper, jobs_buffer, clusters_manager,
            image_storage, photogrammetry, result_storage
        }
    }

    /// Starts the orchestrator main loop
    ///
    /// It uses an Arc<Orchestrator> as it must be able to call the Orchestrator::start_web_server() function
    pub fn start(orchestrator: Arc<Orchestrator>){
        log("Orchestrator", "Starting up");
        log("Orchestrator", "Starting up web server");

        Orchestrator::start_web_server(orchestrator.clone());

        loop {
            if let Err(_) = orchestrator.add_submissions_to_buffer(){

            }

            let mut buffer = orchestrator.jobs_buffer.write().unwrap();

            if buffer.has_buffered_jobs() {
                if let Some(jobs) = buffer.get_pending_submissions(){
                    let mut jobs = jobs;

                    log("Orchestrator", "Selecting cluster");
                    let mut clusters_manager = orchestrator.clusters_manager.write().unwrap();
                    if let Some(selected_cluster) = clusters_manager.select_cluster() {
                        log("Orchestrator", "Selecting jobs to run");
                        let jobs_to_run = orchestrator.select_jobs_to_run(&mut jobs, &selected_cluster);

                        if jobs_to_run.is_some(){
                            let reservation_status = selected_cluster.get_reservation_status();

                            if reservation_status.is_none() {
                                orchestrator.deploy_and_register_photogrammetry_service(selected_cluster);
                            } else {
                                let reservation_status = reservation_status.unwrap();

                                match reservation_status{
                                    ReservationStatus::ResourcesAvailable => {
                                        if let Err(_) = orchestrator.run_jobs(&mut jobs_to_run.unwrap()){

                                        }
                                    }
                                    ReservationStatus::Pending => {
                                        log("Orchestrator", "Waiting for the photogrammetry service node reservation");
                                    }
                                    ReservationStatus::Expired => {
                                        orchestrator.deploy_and_register_photogrammetry_service(selected_cluster);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            buffer.check_timeouts();
            drop(buffer);
            thread::sleep(time::Duration::from_secs(orchestrator.ticks_delay.clone()));
        }
    }

    /// Deploys the photogrammetry service and registers its access information in the services keeper
    fn deploy_and_register_photogrammetry_service(&self, selected_cluster: &mut Cluster) {
        log("Orchestrator", "Deploying photogrammetry service");
        if let Ok(sai) = selected_cluster.deploy_photogrammetry_service() {
            {
                let mut sk = self.services_keeper.write().unwrap();
                sk.register_service(&self.photogrammetry.get_name(), sai);
            }
        }
    }

    /// Starts a web service that listens to the configured port to make the orchestrator able to handle pings from other microservices
    ///
    /// It uses an Arc<Orchestrator> to allow using the orchestrator in different threads, which is necessary to handle TCP connections
    fn start_web_server(orchestrator: Arc<Orchestrator>){
        let o_clone = orchestrator.clone();
        thread::spawn(move || -> Result<(), String>{
            let orchestrator_port = &env::var("ORCHESTRATOR_WS_PORT").unwrap();
            let orchestrator_url = format!("0.0.0.0:{}", orchestrator_port);
            match TcpListener::bind(orchestrator_url.to_string()){
                Ok(callback_listener) => {
                    log("Orchestrator", &format!("Web server listening at {}", orchestrator_url));
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

    /// Fetches the new submissions from the ImageStorageService and adds them to the JobsBuffer if it doesn't have them already
    fn add_submissions_to_buffer(&self) -> Result<(), String>{
        // log("ImageStorage", "Fetching new submissions");
        let get_new_submissions_result = self.image_storage.get_new_submissions();

        if let Err(er) = get_new_submissions_result {
            log_error(&er.to_string());
            return Err(er.to_string());
        }

        let new_submissions = get_new_submissions_result.ok().unwrap();

        if new_submissions.len() > 0 {
            // log("ImageStorage", &format!("Fetched {}", new_submissions.len()));

            let mut buffer = self.jobs_buffer.write().unwrap();
            let image_storage_ai = self.image_storage.get_access_information().unwrap_or(ServiceAccessInformation::new("localhost", 7880, "", ""));
            let image_storage_url = image_storage_ai.get_host();
            let image_storage_port = image_storage_ai.get_port();
            let base_url = format!("http://{}:{}", image_storage_url, image_storage_port);

            for s in new_submissions.into_iter() {
                let photos: Vec<String> = s.photos.iter().map(|s| base_url.clone()+s).collect();
                let photos: Vec<&str> = photos.iter().map(|s| s as &str).collect();

                let job = BufferedJob::new(&None, s.name, &photos, &s.id, SystemTime::now());

                if let false = buffer.submission_exists(&job) {
                    log("JobsBuffer", &format!("Adding job {}", job.to_string()));

                    if let Err(er) = buffer.add_job_or_submission(job) {
                        log_error(&er.to_string());
                    }
                }
            }
        }

        Ok(())
    }

    /// Decides which jobs to run in the list of pending submissions based on available green energy produced for the selected cluster
    fn select_jobs_to_run<'a>(&self, jobs: &'a mut[&'a mut BufferedJob], selected_cluster: &'a Cluster) -> Option<Vec<&'a mut BufferedJob>> {
        let mut jobs_to_run = Vec::new();
        let mut total_complexity = 0f32;

        let energy = selected_cluster.get_green_energy_produced();
        let consumption = selected_cluster.get_current_energy_consumption();
        let available_energy;
        if energy.is_none() {
            available_energy = 0f32;
        } else if consumption.is_none() {
            available_energy = energy.unwrap();
        } else {
            available_energy = energy.unwrap() - consumption.unwrap();
        }

        let node_requirement = selected_cluster.get_node_energy_requirement();

        for job in jobs.iter_mut() {
            if available_energy > 0.001f32 && (total_complexity + node_requirement) * ENERGY_COST_PER_COMPLEXITY_UNIT < available_energy {
                total_complexity += node_requirement;
                jobs_to_run.push(&mut(**job));
            } else if let Ok(time_pending) = SystemTime::now().duration_since(job.submission_date) {
                if time_pending.as_secs() >= self.green_energy_timeout {
                    total_complexity += node_requirement;
                    jobs_to_run.push(&mut (**job));
                }
            }
        }

        if jobs_to_run.len() == 0 {
            return None;
        }
        else {
            return Some(jobs_to_run);
        }
    }

    /// Sends all the received jobs to the photogrammetry service
    fn run_jobs(&self, jobs: &mut[&mut BufferedJob]) -> Result<(), String>{
        log("Orchestrator", &format!("Sending {} job(s) to the photogrammetry service", jobs.len()));
        for job in jobs.iter_mut(){

            log("Photogrammetry", &format!("Creating a job from {} photos", job.photos.len()));
            let orchestrator_port = &env::var("ORCHESTRATOR_WS_PORT").unwrap();
            let job_id = self.photogrammetry.create_job(
                job.submission_id,
                &job.photos,
                &format!("http://{}:{}/photogrammetry/<id>", "orchestrator", orchestrator_port)
            );

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

    /// Handles basic tcp connections
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

        log("Orchestrator", &format!("Path asked: {}", path));
        let mut path_terms = path.split("/");
        // domain
        match path_terms.next() {
            Some(_) => {},
            None => println!("Bad request: [{}] {}", method, path),
        };

        // route level 1
        match path_terms.next() {
            Some(_) => {},
            None => println!("Bad request: [{}] {}", method, path),
        };

        // route level 2 : parameter id
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

        // let response_body = String::from(match self.photogrammetry.get_job(&data, &id){
        //     Ok(x) => x,
        //     Err(_) => {
        //         id = String::from("undefined");
        //         "404"
        //     }
        // });

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

    /// Reacts to a ping from the photogrammetry microservice when a job is done :
    ///
    /// - Sends the result url to the result storage microservice
    /// - Sets the status of the submission to 'Done' in the image storage service
    /// - Removes the submission from the buffer
    fn photogrammetry_callback(&self, id: &str) -> Result<(), ServiceError>{
        let result_url = self.photogrammetry.get_job_result_url(id);
        match result_url {
            Ok(result_url) => {
                let mut buffer = self.jobs_buffer.write().unwrap();
                if let Some(job) = buffer.get_job_by_id(id){
                    log("ResultStorage", &format!("Getting result of submission {}", job.submission_id));
                    if let Err(_) = self.result_storage.post_result(&job.submission_id, &job.name, &result_url){

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

