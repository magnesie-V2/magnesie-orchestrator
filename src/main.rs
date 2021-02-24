//! Magnes.ie project
mod services;
mod jobs_buffer;
mod orchestrator;
mod clusters;
mod ssh_client;
mod meteo_service;
mod simulation;

use std::sync::{Arc, RwLock};

use services::{PhotogrammetryService, ImageStorageService, ServicesKeeper, ServiceAccessInformation};
use jobs_buffer::{JobsBuffer};
use orchestrator::*;
use clusters::{ClustersManager, LocalPhotogrammetry, Grid5000};
use std::time::SystemTime;
use std::{env, thread, time};
use chrono::{DateTime, Utc, Datelike, Timelike};
use crate::services::ResultStorageService;
use simulation::*;

/// If set to true, displays logging in the standard output
const VERBOSE: bool = false;

/// Instantiates the various components and starts the Orchestrator
fn main() -> Result<(), String>{
    let services_keeper = Arc::new(RwLock::new(ServicesKeeper::new()));
    let jobs_buffer = Arc::new(RwLock::new(JobsBuffer::new()));
    let clusters_manager = Arc::new(RwLock::new(ClustersManager::new()));

    add_clusters(&clusters_manager);

    // image storage
    let image_storage_service = ImageStorageService::new(services_keeper.clone())?;
    let input_access_info = ServiceAccessInformation::new(
        "localhost",
        7880,
        "",
        "",
    );
    services_keeper.write().unwrap().register_service("image storage", input_access_info);

    // photogrammetry
    let photogrammetry_service = PhotogrammetryService::new(services_keeper.clone())?;

    // result storage
    let result_storage_service = ResultStorageService::new(services_keeper.clone())?;
    let output_access_info = ServiceAccessInformation::new(
        "localhost",
        7881,
        "",
        "",
    );
    services_keeper.write().unwrap().register_service("result storage", output_access_info);

    let orchestrator = Orchestrator::new(
        86400, // set to 0 to avoid blocking the jos workflow for nothing until Cluster.get_green_energy_produced() is implemented for a cluster
        services_keeper.clone(),
        jobs_buffer.clone(),
        clusters_manager.clone(),
        Arc::new(image_storage_service),
        Arc::new(photogrammetry_service),
        Arc::new(result_storage_service)
    );
    let orchestrator = Arc::new(orchestrator);
    Orchestrator::start_web_server(orchestrator.clone());

    log_energy_headers();
    loop {
        orchestrator.update();

        simulation::log_energy();

        simulation::progress();
        if simulation::should_end(){
            break;
        }
    }
    Ok(())
}

/// Add clusters to the clusters manager
fn add_clusters(clusters_manager: &Arc<RwLock<ClustersManager>>){

    let mut cm_writer = clusters_manager.write().unwrap();
    cm_writer.add_cluster(Box::new(LocalPhotogrammetry::new()));
}

/// Print a message to the standard output
///
/// Example:
/// ```
/// log("MyComponent", "Hello there!");
/// ```
/// Result:
/// ```
/// [22/02/2021 18:40:03][MyComponent] Hello there!
/// ```
pub fn log(component: &str, message: &str){
    if VERBOSE {
        let system_time = SystemTime::now();
        let datetime: DateTime<Utc> = system_time.into();
        let formatted_datetime = datetime.format("%d/%m/%Y %T");
        unsafe{
            let h = CURRENT_TIME_IN_SECONDS.wrapping_div(3600);
            let m = (CURRENT_TIME_IN_SECONDS - h*3600).wrapping_div(60);
            let s = CURRENT_TIME_IN_SECONDS - h*3600 - m*60;
            println!("[{}:{}:{}][{}] {}", h, m, s, component, message);
        }
    }
}

/// Print an error to the standard error
///
/// Example:
/// ```
/// log("MyComponent", "I have a bad feeling about this...");
/// ```
/// Result:
/// ```
/// [22/02/2021 18:40:03][MyComponent] I have a bad feeling about this...
/// ```
pub fn log_error(message: &str) {
    let system_time = SystemTime::now();
    let datetime: DateTime<Utc> = system_time.into();
    let formatted_datetime = datetime.format("%d/%m/%Y %T");

    eprintln!("[{}][ERROR] {}", formatted_datetime, message);
}


