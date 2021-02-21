use crate::services::ServiceAccessInformation;
use crate::clusters::ClusterError;

pub trait Cluster {
    fn get_green_energy_produced(&self) -> Option<f32> {
        None
    }

    fn deploy(&self) -> Result<ServiceAccessInformation, ClusterError> {
        Err(ClusterError::from("Cluster deployment not implemented"))
    }

    fn make_reservation(self) -> String;
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestCluster;
    impl Cluster for TestCluster {
        fn make_reservation(&self) -> String{
            unimplemented!();
        }
    }

    #[test]
    fn test_get_green_energy_produced() {
        let test_cluster = TestCluster;
        assert_eq!(None, test_cluster.get_green_energy_produced());
    }

    #[test]
    fn test_deploy() {
        let test_cluster = TestCluster;
        let deploy_result = test_cluster.deploy();

        assert_eq!(true, deploy_result.is_err());

        let error_message = deploy_result.err().unwrap().to_string(); // unwrap is safe because the previous line ensures this the result is an error

        assert_eq!("[ERROR] Cluster deployment not implemented", error_message);
    }
}
