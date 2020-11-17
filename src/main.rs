pub mod clusters;

use clusters::cluster::*;
use clusters::grid5000::*;

fn main() {
    let g5k: Grid5000 = Grid5000::new(String::from("https://api.grid5000.fr/3.0/?pretty"));

    let g5kResponse = g5k.has_green_energy_available();

    println!("g5k green energy : {r}", r = g5kResponse);
}
