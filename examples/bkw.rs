extern crate lpn;

use lpn::bkw::*;
use lpn::oracle::LpnOracle;

fn main() {
    let mut oracle: LpnOracle = LpnOracle::new(32, 1.0 / 32.0);
    oracle.get_samples(400555);
    let mut secret = oracle.secret.clone();
    secret.truncate(oracle.k as usize);
    let solution = bkw(oracle, 8, 4);

    println!("Found:  {:?}", solution);
    println!("Actual: {:?}", secret);
}
