use crate::services::ServiceAccessInformation;
use crate::clusters::cluster_error::ClusterError;

pub trait Cluster {
    
    fn get_green_energy_produced(&self) -> Option<f32> {
        None 
    }
    
    fn get_current_energy_consumption(&self) -> Option<f32> { None }

    fn deploy_photogrammetry_service(&mut self) -> Result<ServiceAccessInformation, ClusterError>;

    fn free_resources(&mut self) -> Result<(), ClusterError>{
        unimplemented!();
    }

    fn get_reservation_status(&self) -> Option<ReservationStatus>;

    fn get_access_information(&self) -> Option<ServiceAccessInformation>;
}

pub enum ReservationStatus{
    ResourcesAvailable,
    Pending,
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

