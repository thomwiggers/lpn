extern crate lpn;
extern crate m4ri_rust;

use lpn::codes::*;
use lpn::covering_codes::*;
use lpn::gauss::*;
use lpn::lf1::*;
use lpn::oracle::LpnOracle;

use m4ri_rust::friendly::BinVector;

fn main() {
    // setup
    let mut oracle: LpnOracle = LpnOracle::new(15, 1.0 / 8.0);
    oracle.secret = BinVector::from_function(15, |x| x % 2 == 0);
    oracle.get_samples(100_000);

    // sparse secret reduction
    sparse_secret_reduce(&mut oracle);
    let mut secret = oracle.secret.clone();
    let unsps = unsparse_secret(&oracle, &secret);
    println!("unsparsed s:    {:?}", unsps);

    // Do the code reduction
    let code = HammingCode15_11;
    code_reduce(&mut oracle, &code);
    let mut secret = oracle.secret.clone();
    secret.truncate(oracle.k as usize);

    // solve with wht
    let fwht_solution = fwht_solve(oracle.clone());
    println!("Actual:         {:?}", secret);
    println!("Found (FWHT):   {:?}", fwht_solution);

    // solve with pooled gauss
    let gauss_solution = pooled_gauss_solve(oracle.clone());
    println!("Found (Gauss):  {:?}", gauss_solution);
}
