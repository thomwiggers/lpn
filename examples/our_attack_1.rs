extern crate lpn;

use lpn::gauss::*;
use lpn::oracle::LpnOracle;

// k', k: [1, 103, 
// time: 30.320636622242493, 
// memory: 28.58667745647354, 
// samples: 21.957320836393933, 
// samples: 21.957320836393933, 
// steps: 
//   1. drop_reduce (c= 1, k=103); 1,99,30.182832110785544,28.58667745647354,21.957320836393933,21.957320836393933
//   2. xor_drop_reduce (c=1, k=99, opt_k-b1=78), 2,78,29.60343329438861,28.200043891650118,21.91464167278787,21.91464167278787,
//   3. xor_drop_reduce (c=2, k=78, opt_k-b1=57), 4,57,28.91832712084198,27.662173359740482,21.82928334557574,21.82928334557574,
//   4. xor_drop_reduce (c=4, k=57, opt_k-b1=36), 8,36,28.135784626711846,26.82849169259379,21.658566691151478,21.658566691151478,
//   5. codes_reduce
//   6. WHT (c=8, k=36, opt_k_prime=19)']   

fn main() {
    let k = 100;
    let tau = 1.0/8.0;

    let mut oracle: LpnOracle = LpnOracle::new(k, tau);
    oracle.get_samples_drop(2usize.pow(30) as usize, 1);
    println!("Collected samples.");
    //let secret = oracle.secret.clone();
    //let solution = pooled_gauss_solve(oracle);

    //println!("Found:  {:?}", solution);
    //println!("Actual: {:?}", secret);
}
