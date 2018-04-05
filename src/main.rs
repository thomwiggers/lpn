extern crate lpn;
use lpn::oracle::LpnOracle;
use lpn::bkw::*;
use lpn::lf1::*;

fn main() {
    let mut oracle: LpnOracle = LpnOracle::new(32, 1.0/8.0);
    oracle.get_queries(155555);
    let oracle = bkw_reduction(oracle, 4, 8);
    let secret = oracle.secret.clone();
    let solution = lf1_solve(oracle);

    println!("Found:  {:?}", solution);
    println!("Actual: {:?}", secret);
}
