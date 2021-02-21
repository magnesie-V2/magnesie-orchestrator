mod services;
mod jobs_buffer;
mod orchestrator;
mod clusters;

use std::sync::{Arc, RwLock};

use services::{PhotogrammetryService, ImageStorageService, ServicesKeeper, ServiceAccessInformation};
use jobs_buffer::{JobsBuffer};
use orchestrator::*;
use clusters::ClustersManager;

fn main() -> Result<(), String>{
    let services_keeper = Arc::new(RwLock::new(ServicesKeeper::new()));
    let jobs_buffer = Arc::new(RwLock::new(JobsBuffer::new()));
    let clusters_manager = Arc::new(ClustersManager::new());

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

    let orchestrator = Orchestrator::new(
        10,
        0, // set to 0 to avoid blocking the jos workflow for nothing until Cluster.get_green_energy_produced() is implemented for a cluster
        jobs_buffer.clone(),
        clusters_manager.clone(),
        image_storage_service.clone(),
        photogrammetry_service.clone()
    );
    orchestrator.start();

    Ok(())
}
