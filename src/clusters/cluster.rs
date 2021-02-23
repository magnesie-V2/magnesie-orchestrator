use crate::services::ServiceAccessInformation;
use crate::clusters::cluster_error::ClusterError;

pub trait Cluster {
    fn get_green_energy_produced(&self) -> Option<f32> {
        None 
    }
    
    fn get_current_energy_consumption(&self) -> Option<f32> { None }

    fn deploy_photogrammetry_service(&mut self) -> Result<ServiceAccessInformation, ClusterError>;

    fn get_reservation_status(&self) -> Option<ReservationStatus>;

    fn get_access_information(&self) -> Option<ServiceAccessInformation>;

}

pub enum ReservationStatus{
    ResourcesAvailable,
    Pending,
    Expired
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestCluster;
    impl Cluster for TestCluster {
        fn get_reservation_status(&self) -> Option<ReservationStatus> {
            None
        }

        fn get_access_information(&self) -> ServiceAccessInformation {
            unimplemented!()
        }

        fn make_reservation(self) -> String{
            unimplemented!();
        }
    }

    #[test]
    fn test_get_green_energy_produced() {
        let test_cluster = TestCluster;
        assert_eq!(None, test_cluster.get_green_energy_produced());
    }

    #[test]
    fn test_deploy_photogrammetry_service() {
        let test_cluster = TestCluster;
        let deploy_result = test_cluster.deploy_photogrammetry_service();

        assert_eq!(true, deploy_result.is_err());

        let error_message = deploy_result.err().unwrap().to_string(); // unwrap is safe because the previous line ensures this the result is an error

        assert_eq!("Cluster deployment not implemented", error_message);
    }
}
