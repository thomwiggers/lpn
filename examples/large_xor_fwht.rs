/// An example of a reduction chain for LPN_128,1/32
extern crate lpn;

use lpn::bkw::*;
use lpn::lf1::*;
use lpn::oracle::LpnOracle;

fn main() {
    let mut oracle: LpnOracle = LpnOracle::new(128, 1.0 / 32.0);
    oracle.get_samples(200_000);
    xor_reduce(&mut oracle, 8);
    partition_reduce(&mut oracle, 8);
    partition_reduce(&mut oracle, 8);
    partition_reduce(&mut oracle, 8);
    partition_reduce(&mut oracle, 8);
    xor_reduce(&mut oracle, 16);
    partition_reduce(&mut oracle, 8);
    partition_reduce(&mut oracle, 8);
    partition_reduce(&mut oracle, 8);
    partition_reduce(&mut oracle, 8);
    partition_reduce(&mut oracle, 8);
    partition_reduce(&mut oracle, 8);
    partition_reduce(&mut oracle, 8);

    let secret = oracle.secret.as_binvector(oracle.get_k());
    let fwht_solution = fwht_solve(oracle);

    println!("Found (fwht):  {:?}", fwht_solution);
    println!("Actual:        {:?}", secret);
}
