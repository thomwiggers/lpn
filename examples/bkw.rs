extern crate lpn;

use lpn::bkw::*;
use lpn::oracle::LpnOracle;

fn main() {
    let a = 4;
    let b = 8;

    let mut oracle: LpnOracle = LpnOracle::new(32, 1.0 / 32.0);
    oracle.get_samples(2usize.pow(20));
    let mut secret = oracle.secret.clone();
    secret.truncate((oracle.k - (a-1)*b) as usize);
    let solution = bkw(oracle, a, b);

    println!("Found:  {:?}", solution);
    println!("Actual: {:?}", secret);
}
