
//! [1, 151, 45.44865802739947, 38.5544385796504, 31.79955107748693, 31.399775538743466, 
//! 'drop_reduce (c= 1, k=151); 1,138,40.546082016132694,38.5544385796504,31.79955107748693,31.399775538743466,
//! xor_drop_reduce (c=1, k=138, opt_k-b1=108), 2,108,39.98692763531931,38.5544385796504,31.79955107748693,31.79955107748693,
//! xor_drop_reduce (c=2, k=108, opt_k-b1=77), 4,77,39.3192262253413,37.86588869566876,31.59910215497386,31.59910215497386,
//! xor_drop_reduce (c=4, k=77, opt_k-b1=46), 8,46,38.663655599575506,36.721766266004735,31.19820430994772,31.19820430994772,
//! codes_reduce + WHT (c=8, k=46, opt_k_prime=29, code=[46, 29] concatenated code with from ([26,16] Wagner, [20,13] Wagner))']


extern crate lpn;

#[cfg(all(feature = "wagner_26", feature = "guava_20", feature="max_k_191"))]
fn main() {
    use simple_logger::SimpleLogger;
    SimpleLogger::new()
        .with_level(log::LevelFilter::Trace)
        .init()
        .unwrap();

    use lpn::codes::*;
    use lpn::covering_codes::*;
    use lpn::lf1::*;
    use lpn::oracle::LpnOracle;

    let k = 151;
    let tau = 1.0 / 4.0;

    let mut oracle: LpnOracle = LpnOracle::new(k, tau);
    let start_len = 2833180828;
    let k = k as usize;
    oracle.get_samples_drop(start_len + 1000, k - 138);
    log::info!("Collected samples.");
    sparse_secret_reduce(&mut oracle);
    xor_drop_reduce(&mut oracle, 138 - 108, 0);
    xor_drop_reduce(&mut oracle, 108 - 77, 0);
    xor_drop_reduce(&mut oracle, 77 - 46, 0);
    let code = ConcatenatedCode::new(vec![&WagnerCode26_16, &GuavaCode20_13]);
    code_reduce(&mut oracle, &code);

    let secret = oracle.secret.as_binvector(code.dimension());
    let solution = fwht_solve(oracle);
    log::info!("done");

    println!("Found:  {:?}", solution);
    println!("Actual: {:?}", secret);
}

#[cfg(not(all(feature = "wagner_26", feature = "guava_20", feature = "max_k_191")))]
fn main() {
    println!("necessary feature disabled");
}
