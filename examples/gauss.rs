extern crate lpn;

use lpn::gauss::*;
use lpn::oracle::LpnOracle;

fn main() {
    let mut oracle: LpnOracle = LpnOracle::new(32, 1.0 / 32.0);
    oracle.get_samples(40555);
    let secret = oracle.secret.clone();
    let solution = pooled_gauss_solve(oracle);

    println!("Found:  {:?}", solution);
    println!("Actual: {:?}", secret.as_binvector(32));
}
