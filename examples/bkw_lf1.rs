/// This file shows the LF1 attack
extern crate lpn;

use lpn::bkw::*;
use lpn::lf1::*;
use lpn::oracle::LpnOracle;

fn main() {
    // set up oracle
    let mut oracle: LpnOracle = LpnOracle::new(32, 1.0 / 32.0);
    oracle.get_samples(100_555);
    // run BKW reduction 3 times
    // i.e. a=4, b=8
    for _ in 0..3 {
        partition_reduce(&mut oracle, 8);
    }

    // obtain the secret for verification
    let secret = oracle.secret.clone();

    // solve using both LF1 implementations
    let lf1_solution = lf1_solve(oracle.clone());
    let fwht_solution = fwht_solve(oracle);

    println!("Found (lf1):   {:?}", lf1_solution);
    println!("Found (fwht):  {:?}", fwht_solution);
    println!("Actual:        {:b}", secret.get_sample()[0]);
}
