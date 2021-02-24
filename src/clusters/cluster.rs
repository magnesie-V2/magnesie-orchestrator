use crate::services::ServiceAccessInformation;
use crate::clusters::cluster_error::ClusterError;
use crate::CURRENT_TIME_IN_SECONDS;

/// Custom type that represents a ClusterFeatures trait object
pub type Cluster = Box<dyn ClusterFeatures + Send + Sync>;

/// Defines feature shared by all clusters
pub trait ClusterFeatures {
    /// Returns how much energy as been produced since the last iteration of the program's loop
    ///
    /// Mocked with a gaussian function
    fn get_green_energy_produced(&self) -> Option<f32> {
            let e = crate::simulation::get_energy_produced();
            if e <= crate::FLOAT_NIL {
                None
            } else {
                Some(e)
            }
    }

    fn get_node_energy_requirement(&self) -> f32{
        0.0183
    }

    /// Returns how much energy has been consumed since the last iteration of the orchestrator's loop
    fn get_current_energy_consumption(&self) -> Option<f32> {
        let s = crate::PHOTOGRAMMETRY_SERVICE.lock();

        if s.is_err() {
            panic!("Could not retrieve currently used energy");
        }


        let e = s.unwrap().get_currently_used_energy();
        if e <= crate::FLOAT_NIL {
            None
        } else {
            Some(e)
        }
    }

    /// Deploys the photogrammetry service on this cluster
    fn deploy_photogrammetry_service(&mut self) -> Result<ServiceAccessInformation, ClusterError>;

    /// Frees the resources that were reserved with the last deployment
    fn free_resources(&mut self) -> Result<(), ClusterError>{
        unimplemented!();
    }

    /// Returns the current status of the resources reservations
    fn get_reservation_status(&self) -> Option<ReservationStatus>;

    /// Returns the access information of the deployed photogrammetry service
    fn get_access_information(&self) -> Option<ServiceAccessInformation>;
}

/// Defines the possible statuses of a cluster resources reservation
pub enum ReservationStatus{
    /// The resources can be used
    ResourcesAvailable,
    /// The resources can't be used yet
    Pending,
    /// The resources have expired and can't be used anymore
    Expired,
}

impl PartialEq for ReservationStatus{
    fn eq(&self, other: &Self) -> bool {
        match self{
            ReservationStatus::ResourcesAvailable => match other{
                ReservationStatus::ResourcesAvailable => true,
                _ => false
            }
            ReservationStatus::Pending => match other{
                ReservationStatus::Pending => true,
                _ => false
            }
            ReservationStatus::Expired => match other{
                ReservationStatus::Expired => true,
                _ => false
            }
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Clone for ReservationStatus{
    fn clone(&self) -> Self {
        match self{
            ReservationStatus::ResourcesAvailable => ReservationStatus::ResourcesAvailable,
            ReservationStatus::Pending => ReservationStatus::Pending,
            ReservationStatus::Expired => ReservationStatus::Expired,
        }
    }
}

