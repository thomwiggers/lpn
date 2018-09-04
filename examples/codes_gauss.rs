extern crate lpn;
extern crate time;
extern crate rayon;

use lpn::oracle::*;
use lpn::gauss::*;
use lpn::covering_codes::*;
use lpn::codes::*;

fn main() {
    let repcode5 = RepetitionCode::new(5);
    let idcode = IdentityCode::new(1);
    let mut subcodes: Vec<&dyn BinaryCode> = vec![&repcode5; 51];
    subcodes.push(&idcode);
    let concatenated = ConcatenatedCode::new(subcodes.clone());
    // let stgen = StGenCode::new(subcodes.clone(), 3, 400, 3, 3);

    println!(
        "Bias of concatenated code: {:e}",
        concatenated.bias(1.0 - 2.0 * 1.0 / 8.0)
    );
    //println!("Bias of stgen code: {:e}", stgen.bias(1.0 - 2.0 * 1.0 / 8.0));

    let mut oracle: LpnOracle = LpnOracle::new(256, 1.0 / 8.0);
    oracle.get_samples(900);
    reduce_sparse_secret(&mut oracle);

    let secret = oracle.secret.clone();
    code_reduction(&mut oracle, concatenated);

    let solution = pooled_gauss_solve(oracle);

    println!("Found:  {:?}", solution);
    println!("Actual: {:?}", secret);

}
