use crate::clusters::cluster::Cluster;
use crate::clusters::cluster_error::ClusterError;
use crate::services::ServiceAccessInformation;

pub struct LocalPhotogrammetry;

impl Cluster for LocalPhotogrammetry{
    fn deploy_photogrammetry_service(&self) -> Result<ServiceAccessInformation, ClusterError> {
        Ok(ServiceAccessInformation::new(
            "localhost",
            7879,
            "",
            ""
        ))
    }

    fn make_reservation(&self) -> String {
        "".to_string()
    }
}
