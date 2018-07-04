extern crate lpn;

use lpn::bkw::*;
use lpn::lf1::*;
use lpn::oracle::LpnOracle;

fn main() {
    let mut oracle: LpnOracle = LpnOracle::new(128, 1.0 / 32.0);
    oracle.get_queries(2000);
    oracle = xor_reduction(oracle, 8);
    oracle = bkw_reduction(oracle, 2, 8);
    oracle = bkw_reduction(oracle, 2, 8);
    oracle = bkw_reduction(oracle, 2, 8);
    oracle = bkw_reduction(oracle, 2, 8);
    oracle = xor_reduction(oracle, 8);
    oracle = xor_reduction(oracle, 8);
    oracle = bkw_reduction(oracle, 2, 8);
    oracle = bkw_reduction(oracle, 2, 8);
    oracle = bkw_reduction(oracle, 2, 8);
    oracle = bkw_reduction(oracle, 2, 8);
    oracle = bkw_reduction(oracle, 2, 8);

    let secret = oracle.secret.clone();
    let fwht_solution = fwht_solve(oracle);

    println!("Found (fwht):  {:?}", fwht_solution);
    println!("Actual:        {:?}", secret);
}
