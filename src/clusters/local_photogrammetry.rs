use crate::clusters::{Cluster, ClusterError, ReservationStatus};
use crate::services::ServiceAccessInformation;

pub struct LocalPhotogrammetry{
    reservation_status: Option<ReservationStatus>,
}

impl LocalPhotogrammetry{
    pub fn new() -> LocalPhotogrammetry{
        LocalPhotogrammetry {
            reservation_status: None
        }
    }
}

impl Cluster for LocalPhotogrammetry{
    fn deploy_photogrammetry_service(&mut self) -> Result<ServiceAccessInformation, ClusterError> {
        self.reservation_status = Some(ReservationStatus::ResourcesAvailable);
        Ok(self.get_access_information().unwrap())
    }

    fn get_reservation_status(&self) -> Option<ReservationStatus> {
        self.reservation_status.clone()
    }

    fn get_access_information(&self) -> Option<ServiceAccessInformation> {
        Some(ServiceAccessInformation::new(
            "localhost",
            7879,
            "",
            ""
        ))
    }
}
