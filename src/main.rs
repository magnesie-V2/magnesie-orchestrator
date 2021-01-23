mod services;
mod jobs_buffer;

use std::sync::{Arc, RwLock};

use services::{PhotogrammetryService, ServicesKeeper};
use jobs_buffer::{JobsBuffer};

fn main() -> Result<(), String>{
    let services_keeper = Arc::new(RwLock::new(ServicesKeeper::new()));
    let jobs_buffer = Arc::new(RwLock::new(JobsBuffer::new()));

    let photogrammetry_service = PhotogrammetryService::new(services_keeper.clone())?;

    Ok(())
}

