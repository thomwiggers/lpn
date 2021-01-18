extern crate lpn;

use lpn::bkw::partition_reduce;
use lpn::gauss::*;
use lpn::oracle::LpnOracle;

// Attack on LPN decoded
// k = 135, tau = 1/4
// First get 2^33 samples with k_1 = 3 bits fixed
// Eliminate 91 bits with BKW with block size 31 / 3 rounds
// Solve LPN_17,255/512 with Gauss

fn main() {
    let k = 135;
    let tau = 1.0 / 4.0;

    println!(
        "Sizeof sample: {:?}",
        ::std::mem::size_of::<::lpn::oracle::Sample>()
    );

    let mut oracle: LpnOracle = LpnOracle::new(k, tau);
    oracle.get_samples_drop(2usize.pow(32) as usize, 3);
    println!("Collected samples.");

    partition_reduce(&mut oracle, 31);
    partition_reduce(&mut oracle, 31);
    partition_reduce(&mut oracle, 31);

    let secret = oracle.secret.as_binvector(oracle.get_k());
    let solution = pooled_gauss_solve(oracle);

    println!("Found:  {:?}", solution);
    println!("Actual: {:?}", secret);
}
