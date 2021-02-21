use super::Cluster;

pub struct ClustersManager{
    clusters: Vec<Box<dyn Cluster>>
}

impl ClustersManager{
    pub fn new() -> ClustersManager{
        ClustersManager {
            clusters: Vec::new()
        }
    }

    pub fn add_cluster(&mut self, cluster: Box<dyn Cluster>) {
        self.clusters.push(cluster);
    }

    pub fn has_clusters(&self) -> bool{
        self.clusters.len() > 0
    }

    pub fn select_cluster(&self) -> Option<&Box<dyn Cluster>> {
        if !self.has_clusters() {
            return None;
        }

        let mut selected_cluster = self.clusters.get(0).unwrap(); // unwrap safe as we know there is at least one cluster

        for cluster in self.clusters.iter(){
            let cluster_energy = cluster.get_green_energy_produced();
            if cluster_energy.is_none() {
                continue;
            }

            let selected_cluster_energy = selected_cluster.get_green_energy_produced();
            if selected_cluster_energy.is_none() {
                selected_cluster = cluster;
                continue;
            }

            if cluster_energy.unwrap() > selected_cluster_energy.unwrap(){
                selected_cluster = cluster;
            }
        }

        Some(selected_cluster)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestCluster;
    impl Cluster for TestCluster{}

    #[test]
    fn test_add_cluster(){
        let mut clusters_manager = ClustersManager::new();
        let test_cluster = TestCluster;

        assert_eq!(0, clusters_manager.clusters.len());
        clusters_manager.add_cluster(Box::new(test_cluster));
        assert_eq!(1, clusters_manager.clusters.len());
    }

    #[test]
    fn test_has_clusters(){
        let mut clusters_manager = ClustersManager::new();
        let test_cluster = TestCluster;

        assert_eq!(false, clusters_manager.has_clusters());
        clusters_manager.clusters.push(Box::new(test_cluster));
        assert_eq!(true, clusters_manager.has_clusters());
    }

    #[test]
    fn test_select_cluster(){
        let mut clusters_manager = ClustersManager::new();
        let test_cluster = TestCluster;

        assert_eq!(true, clusters_manager.select_cluster().is_none());
        clusters_manager.add_cluster(Box::new(test_cluster));
        assert_eq!(false, clusters_manager.select_cluster().is_none());
    }
}
