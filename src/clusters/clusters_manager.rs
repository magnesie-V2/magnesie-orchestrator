use super::Cluster;

pub struct ClustersManager{
    clusters: Vec<Box<dyn Cluster + Send + Sync>>
}

impl ClustersManager{
    pub fn new() -> ClustersManager{
        ClustersManager {
            clusters: Vec::new()
        }
    }

    pub fn add_cluster(&mut self, cluster: Box<dyn Cluster + Send + Sync>) {
        self.clusters.push(cluster);
    }

    pub fn has_clusters(&self) -> bool{
        self.clusters.len() > 0
    }

    pub fn select_cluster(&mut self) -> Option<&mut Box<dyn Cluster + Send + Sync>> {
        if !self.has_clusters() {
            return None;
        }

        // TODO
        // let selected_cluster = self.clusters.get_mut(0).unwrap(); // unwrap safe as we know there is at least one cluster

        /*
        for cluster in self.clusters.iter_mut(){
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
        }*/

        Some(self.clusters.get_mut(0).unwrap())
    }
}

