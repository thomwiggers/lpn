extern crate lpn;

use lpn::bkw::*;
use lpn::lf1::lf1_solve;
use lpn::oracle::LpnOracle;

fn main() {
    let mut oracle: LpnOracle = LpnOracle::new(32, 1.0 / 32.0);
    oracle.get_queries(100555);
    let oracle = bkw_reduction(oracle, 8, 4);
    let mut secret = oracle.secret.clone();
    secret.truncate(oracle.k as usize);
    let solution = lf1_solve(oracle);

    println!("Found:  {:?}", solution);
    println!("Actual: {:?}", secret);
}
