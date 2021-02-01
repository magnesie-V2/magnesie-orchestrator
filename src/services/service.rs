use std::sync::{Arc, RwLock};

use super::service_error::ServiceError;
use super::remote::{ServicesKeeper, ServiceAccessInformation};

pub trait Service {
    fn get_name(&self) -> String;
    fn get_services_keeper(&self) -> Arc<RwLock<ServicesKeeper>>;

    fn get_access_information(&self) -> Result<ServiceAccessInformation, ServiceError> {
        let services_keeper = self.get_services_keeper();
        let services_keeper = services_keeper.read().unwrap();

        let sai = services_keeper.get_service(&self.get_name());

        match sai{
            Some(sai) => Ok(ServiceAccessInformation::new(
                sai.get_host(),
                sai.get_port().clone(),
                sai.get_username(),
                sai.get_password()
            )),
            _ => Err(ServiceError::from(format!("No {} service available", self.get_name())))
        }
    }
}

