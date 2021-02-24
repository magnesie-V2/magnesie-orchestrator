/// This type allows using trait objects while keeping the ClustersManager readable
use super::Cluster;

pub struct ClustersManager{
    clusters: Vec<Cluster>
}

impl ClustersManager{
    pub fn new() -> ClustersManager{
        ClustersManager {
            clusters: Vec::new()
        }
    }

    pub fn add_cluster(&mut self, cluster: Cluster) {
        self.clusters.push(cluster);
    }

    pub fn has_clusters(&self) -> bool{
        self.clusters.len() > 0
    }

    pub fn select_cluster(&mut self) -> Option<&mut Cluster> {
        if !self.has_clusters() {
            return None;
        }

        Some(self.clusters.get_mut(0).unwrap())
    }
}

