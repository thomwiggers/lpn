extern crate lpn;
use lpn::oracle::LpnOracle;
use lpn::bkw::*;
#[allow(unused_imports)]
use lpn::lf1::*;
use lpn::gauss::*;

fn main() {
    let mut oracle: LpnOracle = LpnOracle::new(32, 1.0/8.0);
    oracle.get_queries(155555);
    let oracle = bkw_reduction(oracle, 4, 8);
    let mut secret = oracle.secret.clone();
    secret.truncate(oracle.k as usize);
    let solution = pooled_gauss_solve(oracle);

    println!("Found:  {:?}", solution);
    println!("Actual: {:?}", secret);
}
