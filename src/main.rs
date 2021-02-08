mod services;
mod jobs_buffer;
mod orchestrator;

use std::sync::{Arc, RwLock};

use services::{PhotogrammetryService, ImageStorageService, ServicesKeeper};
use jobs_buffer::{JobsBuffer};
use orchestrator::*;

fn main() -> Result<(), String>{
    let services_keeper = Arc::new(RwLock::new(ServicesKeeper::new()));
    let jobs_buffer = Arc::new(RwLock::new(JobsBuffer::new()));

    let image_storage_service = ImageStorageService::new(services_keeper.clone())?;
    let photogrammetry_service = PhotogrammetryService::new(services_keeper.clone())?;

    let orchestrator = Orchestrator::new(2, 60);
    orchestrator.start();

    Ok(())
}

