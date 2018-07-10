extern crate lpn;

use lpn::bkw::*;
use lpn::oracle::LpnOracle;

fn main() {
    let mut oracle: LpnOracle = LpnOracle::new(32, 1.0 / 32.0);
    oracle.get_queries(40555);
    bkw_reduction(&mut oracle, 8, 4);
    let mut secret = oracle.secret.clone();
    secret.truncate(oracle.k as usize);
    let solution = bkw_solve(oracle);

    println!("Found:  {:?}", solution);
    println!("Actual: {:?}", secret);
}
