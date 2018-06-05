extern crate lpn;

use lpn::gauss::*;
use lpn::oracle::LpnOracle;

fn main() {
    let mut oracle: LpnOracle = LpnOracle::new(32, 0f64);
    oracle.get_queries(40555);
    let secret = oracle.secret.clone();
    let solution = pooled_gauss_solve(oracle);

    println!("Found:  {:?}", solution);
    println!("Actual: {:?}", secret);
}
