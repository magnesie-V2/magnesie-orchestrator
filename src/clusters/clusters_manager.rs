use super::Cluster;

/// The cluster manager keeps the list of all the available clusters
///
/// Its purpose is to select the most suited cluster to run jobs
pub struct ClustersManager{
    clusters: Vec<Cluster>
}

impl ClustersManager{
    /// Creates a ClustersManager struct
    pub fn new() -> ClustersManager{
        ClustersManager {
            clusters: Vec::new()
        }
    }

    /// Adds a cluster to the list
    pub fn add_cluster(&mut self, cluster: Cluster) {
        self.clusters.push(cluster);
    }

    /// Checks whether there are clusters in the list
    pub fn has_clusters(&self) -> bool{
        self.clusters.len() > 0
    }

    /// Selects the best cluster in the list
    ///
    /// TODO: cluster selection based on available green energy
    pub fn select_cluster(&mut self) -> Option<&mut Cluster> {
        if !self.has_clusters() {
            return None;
        }

        Some(self.clusters.get_mut(0).unwrap())
    }
}

