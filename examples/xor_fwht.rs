/// Example of the LF2 xor_reduce reduction and WHT solving method.

extern crate lpn;

use lpn::lf1::*;
use lpn::oracle::LpnOracle;

fn main() {
    let mut oracle: LpnOracle = LpnOracle::new(32, 1.0 / 32.0);
    oracle.get_samples(1000);
    xor_reduce(&mut oracle, 8);
    xor_reduce(&mut oracle, 8);
    xor_reduce(&mut oracle, 8);
    let mut secret = oracle.secret.clone();
    secret.truncate(oracle.k as usize);
    // both implementations of LF1's WHT
    let lf1_solution = lf1_solve(oracle.clone());
    let fwht_solution = fwht_solve(oracle);

    println!("Found (lf1):   {:?}", lf1_solution);
    println!("Found (fwht):  {:?}", fwht_solution);
    println!("Actual:        {:?}", secret);
}
