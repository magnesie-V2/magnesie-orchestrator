use crate::clusters::{Cluster, ClusterError, ReservationStatus};
use crate::services::ServiceAccessInformation;

pub struct LocalPhotogrammetry;

impl Cluster for LocalPhotogrammetry{
    fn deploy_photogrammetry_service(&self) -> Result<ServiceAccessInformation, ClusterError> {
        Ok(self.get_access_information().unwrap())
    }

    fn get_reservation_status(&self) -> Option<ReservationStatus> {
        Some(ReservationStatus::ResourcesAvailable)
    }

    fn get_access_information(&self) -> Option<ServiceAccessInformation> {
        Some(ServiceAccessInformation::new(
            "localhost",
            7879,
            "",
            ""
        ))
    }

    fn make_reservation(self) -> String {
        "".to_string()
    }
}
