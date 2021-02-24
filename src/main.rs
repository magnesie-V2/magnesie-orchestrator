mod services;
mod jobs_buffer;
mod orchestrator;
mod clusters;
mod ssh_client;
mod meteo_service;

use std::sync::{Arc, RwLock};

use services::{PhotogrammetryService, ImageStorageService, ServicesKeeper, ServiceAccessInformation};
use jobs_buffer::{JobsBuffer};
use orchestrator::*;
use clusters::{ClustersManager, LocalPhotogrammetry, Grid5000};
use std::time::SystemTime;
use std::env;
use chrono::{DateTime, Utc};
use crate::services::ResultStorageService;

fn main() -> Result<(), String>{

    let args: Vec<String> = env::args().collect();

    let services_keeper = Arc::new(RwLock::new(ServicesKeeper::new()));
    let jobs_buffer = Arc::new(RwLock::new(JobsBuffer::new()));
    let clusters_manager = Arc::new(RwLock::new(ClustersManager::new()));
    
    if args.len() > 1 {
        log("Main", "Launch parameters found, adding Grid5000 cluster");
        let username : &str = &args[1];
        let password : &str = &args[2];
        let site : &str = &args[3];
        let walltime : &str = &args[4];
        add_grid5000_cluster(&clusters_manager, username, password, site, walltime);
    }
    else {
        add_clusters(&clusters_manager);
    }


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
        10,
        0, // set to 0 to avoid blocking the jos workflow for nothing until Cluster.get_green_energy_produced() is implemented for a cluster
        services_keeper.clone(),
        jobs_buffer.clone(),
        clusters_manager.clone(),
        Arc::new(image_storage_service),
        Arc::new(photogrammetry_service),
        Arc::new(result_storage_service)
    );
    Orchestrator::start(Arc::new(orchestrator));
    Ok(())
}

fn add_clusters(clusters_manager: &Arc<RwLock<ClustersManager>>){

    let mut cm_writer = clusters_manager.write().unwrap();
    cm_writer.add_cluster(Box::new(LocalPhotogrammetry::new()));
}

fn add_grid5000_cluster(clusters_manager: &Arc<RwLock<ClustersManager>>, username : &str, password : &str, site : &str, walltime : &str){

    let mut cm_writer = clusters_manager.write().unwrap();
    
    let grid5000_cluster  = Grid5000::new(String::from(username),
    String::from(password),
    String::from(site),
    String::from(walltime));
    
    cm_writer.add_cluster(Box::new(grid5000_cluster));
}

pub fn log(component: &str, message: &str){
    let system_time = SystemTime::now();
    let datetime: DateTime<Utc> = system_time.into();
    let formatted_datetime = datetime.format("%d/%m/%Y %T");

    println!("[{}][{}] {}", formatted_datetime, component, message);
}

pub fn log_error(message: &str) {
    let system_time = SystemTime::now();
    let datetime: DateTime<Utc> = system_time.into();
    let formatted_datetime = datetime.format("%d/%m/%Y %T");

    eprintln!("[{}][ERROR] {}", formatted_datetime, message);
}
