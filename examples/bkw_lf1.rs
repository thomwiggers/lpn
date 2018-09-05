extern crate lpn;

use lpn::bkw::*;
use lpn::lf1::*;
use lpn::oracle::LpnOracle;

fn main() {
    let mut oracle: LpnOracle = LpnOracle::new(32, 1.0 / 32.0);
    oracle.get_samples(100555);
    for _ in 0..3 {
        partition_reduce(&mut oracle, 8);
    }
    let secret = oracle.secret.clone();
    let lf1_solution = lf1_solve(oracle.clone());
    let fwht_solution = fwht_solve(oracle);

    println!("Found (lf1):   {:?}", lf1_solution);
    println!("Found (fwht):  {:?}", fwht_solution);
    println!("Actual:        {:?}", secret);
}
