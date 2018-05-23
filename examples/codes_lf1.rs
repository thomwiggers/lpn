extern crate lpn;

use lpn::codes::HammingCode7_4;
use lpn::covering_codes::*;
use lpn::lf1::*;
use lpn::oracle::LpnOracle;

fn main() {
    let mut oracle: LpnOracle = LpnOracle::new(7, 1.0 / 32.0);
    oracle.get_queries(40555);
    let oracle = reduce_covering_codes(oracle, HammingCode7_4);
    let mut secret = oracle.secret.clone();
    secret.truncate(oracle.k as usize);
    let solution = lf1_solve(oracle);

    println!("Found:  {:?}", solution);
    println!("Actual: {:?}", secret);
}
