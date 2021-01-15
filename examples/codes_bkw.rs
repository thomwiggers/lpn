/// This file demonstrates the combination of the covering codes
/// reduction with the BKW majority solving method.
extern crate lpn;

use lpn::bkw::*;
use lpn::codes::*;
use lpn::covering_codes::*;
use lpn::oracle::LpnOracle;

fn main() {
    // setup oracle
    let mut oracle: LpnOracle = LpnOracle::new(25, 1.0 / 16.0);
    oracle.get_samples(800_555);

    // sparse secret transformation
    sparse_secret_reduce(&mut oracle);

    //use code reduction
    let code = ConcatenatedCode::new(vec![&HammingCode15_11, &HammingCode7_4, &HammingCode3_1]);
    code_reduce(&mut oracle, &code);

    let secret = oracle.secret.as_binvector(oracle.get_k());
    // obtain solution
    let solution = majority(oracle);

    println!("Found:  {:?}", solution);
    println!("Actual: {:?}", secret);
}
