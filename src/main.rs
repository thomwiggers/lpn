#![allow(unused_imports)]
extern crate lpn;

use lpn::bkw::*;
use lpn::codes::*;
use lpn::covering_codes::*;
use lpn::gauss::*;
use lpn::lf1::*;
use lpn::oracle::LpnOracle;

fn main() {
    let mut oracle: LpnOracle = LpnOracle::new(31, 1.0 / 8.0);
    oracle.get_queries(1055555);
    let oracle = reduce_covering_codes(oracle, HammingCode31_26);
    let mut secret = oracle.secret.clone();
    secret.truncate(oracle.k as usize);
    let solution = pooled_gauss_solve(oracle);

    println!("Found:  {:?}", solution);
    println!("Actual: {:?}", secret);
}
