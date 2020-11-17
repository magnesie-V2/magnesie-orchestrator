pub mod cluster;

use cluster::Cluster::*;
use cluster::Grid5000::*;

fn main() {
    let g5k: Grid5000 = Grid5000::new(String::from("https://www.cluster.io/"));

    let g5kResponse = g5k.has_green_energy_available();

    println!("g5k green energy : {r}", r = g5kResponse);
}
