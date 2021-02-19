pub trait Cluster {
    fn new(uri: &str) -> Self;
    fn has_green_energy_available(self) -> bool;
    fn make_reservation(self) -> String;
}
