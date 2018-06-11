extern crate lpn;

use lpn::codes::*;
use lpn::covering_codes::*;
use lpn::gauss::*;
use lpn::oracle::LpnOracle;
use lpn::lf1::*;

fn main() {
    let mut oracle: LpnOracle = LpnOracle::new(25, 1.0 / 32.0);
    oracle.get_queries(100555);
    let code = ConcatenatedCode::new(vec![&HammingCode15_11, &HammingCode7_4, &HammingCode3_1]);
    let oracle = reduce_sparse_secret(oracle);
    let oracle = code_reduction(oracle, code);
    let mut secret = oracle.secret.clone();
    secret.truncate(oracle.k as usize);
    println!("Actual:        {:?}", secret);
    let solution = fwht_solve(oracle.clone());
    println!("Found (FWHT):  {:?}", solution);
    let solution = pooled_gauss_solve(oracle);
    println!("Found (Gauss): {:?}", solution);

}
