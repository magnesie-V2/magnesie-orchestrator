use std::sync::{Arc, RwLock};

use super::service_error::ServiceError;
use super::remote::{ServicesKeeper, ServiceAccessInformation};

/// This trait represents functionalities shared by all services clients
pub trait Service {
    /// Returns the name of this service
    fn get_name(&self) -> String;

    /// Returns an Arc<RwLock<>> to the service keeper
    fn get_services_keeper(&self) -> Arc<RwLock<ServicesKeeper>>;

    /// Returns the current access information of this service
    fn get_access_information(&self) -> Result<ServiceAccessInformation, ServiceError> {
        let services_keeper = self.get_services_keeper();
        let services_keeper = services_keeper.read().unwrap();

        if let Some(sai) = services_keeper.get_service(&self.get_name()){
            return Ok(ServiceAccessInformation::new(
                sai.get_host(),
                sai.get_port().clone(),
                sai.get_username(),
                sai.get_password()
            ));
        }

        Err(ServiceError::from(format!("No {} service available", self.get_name())))
    }
}

