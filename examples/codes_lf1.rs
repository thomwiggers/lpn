extern crate lpn;
extern crate m4ri_rust;

use lpn::codes::*;
use lpn::covering_codes::*;
use lpn::gauss::*;
use lpn::lf1::*;
use lpn::oracle::LpnOracle;

use m4ri_rust::friendly::BinVector;

fn main() {
    let mut oracle: LpnOracle = LpnOracle::new(15, 1.0 / 8.0);
    oracle.secret = BinVector::from_function(15, |x| x % 2 == 0);
    oracle.get_samples(100000);
    //let code = ConcatenatedCode::new(vec![&HammingCode15_11, &HammingCode7_4, &HammingCode3_1]);
    let code = HammingCode15_11;
    sparse_secret_reduce(&mut oracle);
    let mut secret = oracle.secret.clone();
    secret.truncate(oracle.k as usize);
    let unsps = unsparse_secret(&oracle, &secret);
    println!("unsparsed s:    {:?}", unsps);

    code_reduce(&mut oracle, &code);
    let mut secret = oracle.secret.clone();
    secret.truncate(oracle.k as usize);

    let fwht_solution = fwht_solve(oracle.clone());
    println!("Actual:         {:?}", secret);
    println!("Found (FWHT):   {:?}", fwht_solution);

    let gauss_solution = pooled_gauss_solve(oracle.clone());
    println!("Found (Gauss):  {:?}", gauss_solution);
}
