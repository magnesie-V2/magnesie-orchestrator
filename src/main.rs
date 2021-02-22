mod services;
mod jobs_buffer;
mod orchestrator;
mod clusters;

use std::sync::{Arc, RwLock};

use services::{PhotogrammetryService, ImageStorageService, ServicesKeeper, ServiceAccessInformation};
use jobs_buffer::{JobsBuffer};
use orchestrator::*;
use clusters::ClustersManager;
use crate::clusters::LocalPhotogrammetry;
use std::time::SystemTime;
use chrono::{DateTime, Utc};

fn main() -> Result<(), String>{
    let services_keeper = Arc::new(RwLock::new(ServicesKeeper::new()));
    let jobs_buffer = Arc::new(RwLock::new(JobsBuffer::new()));
    let mut clusters_manager = Arc::new(RwLock::new(ClustersManager::new()));
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
        image_storage_service.clone(),
        photogrammetry_service.clone()
    );
    orchestrator.start();

    Ok(())
}

fn add_clusters(clusters_manager: &Arc<RwLock<ClustersManager>>){
    clusters_manager.write().unwrap().add_cluster(Box::new(LocalPhotogrammetry));
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

    println!("[{}][ERROR] {}", formatted_datetime, message);
}
