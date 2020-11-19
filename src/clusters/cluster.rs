pub trait Cluster {
    fn new(uri: String) -> Self;
    fn has_green_energy_available(self) -> bool;
}
