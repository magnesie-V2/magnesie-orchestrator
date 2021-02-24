//! Contains clients for all the microservices and the ServiceKeeper, which keeps track of where the microservices are

pub mod service;
pub use service::*;

pub mod service_error;
pub use service_error::*;

pub mod remote;
pub use remote::*;

pub mod photogrammetry;
pub use photogrammetry::*;

pub mod image_storage;
pub use image_storage::*;

pub mod result_storage;
pub use result_storage::*;

