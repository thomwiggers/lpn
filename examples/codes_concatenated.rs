extern crate lpn;
extern crate m4ri_rust;

use m4ri_rust::friendly::BinVector;

use lpn::codes::*;
use lpn::covering_codes::*;
use lpn::gauss::*;
use lpn::lf1::*;
use lpn::oracle::LpnOracle;

fn main() {
    let secret = BinVector::from_bools(&[
        false, true, true, false, true, true, true, false, true,
        true, false, false, true, false, true, true, false, true,
        true, true, false, true, false, true, true,
    ]);
    assert_eq!(secret.len(), 25);

    // setup oracle
    let mut oracle: LpnOracle = LpnOracle::new_with_secret(secret, 1.0 / 8.0);
    oracle.get_samples(100_555);

    // sparse secret transformation
    sparse_secret_reduce(&mut oracle);

    // code reductoin
    let code = ConcatenatedCode::new(vec![&HammingCode15_11, &HammingCode7_4, &HammingCode3_1]);
    code_reduce(&mut oracle, &code);

    // obtain secret secret that's been transformed by the code reduction
    let mut secret = oracle.secret.clone();
    secret.truncate(oracle.k as usize);

    // get solutions
    println!("Actual:        {:?}", secret);
    let solution = fwht_solve(oracle.clone());
    println!("Found (FWHT):  {:?}", solution);
    let solution = pooled_gauss_solve(oracle);
    println!("Found (Gauss): {:?}", solution);
}
