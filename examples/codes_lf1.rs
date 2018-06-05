extern crate lpn;

use lpn::codes::*;
use lpn::covering_codes::*;
use lpn::lf1::*;
use lpn::oracle::LpnOracle;

fn main() {
    let mut oracle: LpnOracle = LpnOracle::new(25, 1.0 / 8.0);
    oracle.get_queries(400555);
    let code = ConcatenatedCode::new(vec![&HammingCode15_11, &HammingCode7_4, &HammingCode3_1]);
    let oracle = reduce_sparse_secret(oracle);
    let oracle = reduce_covering_codes(oracle, code);
    let mut secret = oracle.secret.clone();
    secret.truncate(oracle.k as usize);
    let solution = lf1_solve(oracle);

    println!("Found:  {:?}", solution);
    println!("Actual: {:?}", secret);
}
