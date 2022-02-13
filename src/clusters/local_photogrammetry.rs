use std::env;
use crate::clusters::{ClusterError, ReservationStatus, ClusterFeatures};
use crate::services::ServiceAccessInformation;

/// This represents a "fake" cluster, in a context where the photogrammetry service is already running on localhost
pub struct LocalPhotogrammetry{
    reservation_status: Option<ReservationStatus>,
}

impl LocalPhotogrammetry{
    /// Creates a LocalPhotogrammetry struct
    pub fn new() -> LocalPhotogrammetry{
        LocalPhotogrammetry {
            reservation_status: None
        }
    }
}

impl ClusterFeatures for LocalPhotogrammetry{
    fn get_node_energy_requirement(&self) -> f32 {
        0f32
    }

    fn deploy_photogrammetry_service(&mut self) -> Result<ServiceAccessInformation, ClusterError> {
        self.reservation_status = Some(ReservationStatus::ResourcesAvailable);
        Ok(self.get_access_information().unwrap())
    }

    fn get_reservation_status(&self) -> Option<ReservationStatus> {
        self.reservation_status.clone()
    }

    fn get_access_information(&self) -> Option<ServiceAccessInformation> {
        Some(ServiceAccessInformation::new(
            &env::var("PHOTOGRAMMETRY_WS_HOST").unwrap(),
            env::var("PHOTOGRAMMETRY_WS_PORT").unwrap().parse::<u16>().unwrap(),
            "",
            ""
        ))
    }
}
