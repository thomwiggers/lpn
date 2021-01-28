extern crate lpn;

/// [1, 200, 44.40156524284424, 38.93227219400191, 31.93227219400191, 31.233068048500478,
/// 'drop_reduce (c= 1, k=200); 1,188,41.2230069040311,38.93227219400191,31.93227219400191,31.233068048500478,
/// xor_drop_reduce (c=1, k=188, opt_k-b1=158), 2,158,40.86954992408068,38.93227219400191,31.93227219400191,31.466136097000955,
/// xor_drop_reduce (c=2, k=158, opt_k-b1=128), 4,128,40.308255795927266,38.93227219400191,31.93227219400191,31.93227219400191,
/// xor_drop_reduce (c=4, k=128, opt_k-b1=97), 8,97,39.60623381197313,38.46445723019095,31.864544388003825,31.864544388003825,
/// xor_drop_reduce (c=8, k=97, opt_k-b1=66), 16,66,38.73531593320804,37.7734828953661,31.72908877600765,31.72908877600765,
/// codes_reduce + WHT (c=16, k=66, opt_k_prime=28, code=[66, 28] concatenated code with from ([19,6] rnd150926, [19,6] rnd150926, [3,1] repetition, [25,15] Wagner))']    
#[cfg(all(feature = "wagner_25", feature = "bogosrnd_19"))]
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

    let k = 200;
    let tau = 1.0 / 8.0;

    let mut oracle: LpnOracle = LpnOracle::new(k, tau);
    let start_len = 2524005746;
    let k = k as usize;
    oracle.get_samples_drop(start_len + 1000, k - 188);
    log::info!("Collected samples.");
    sparse_secret_reduce(&mut oracle);
    xor_drop_reduce(&mut oracle, 188 - 158, 0);
    xor_drop_reduce(&mut oracle, 158 - 128, 0);
    xor_drop_reduce(&mut oracle, 128 - 97, 0);
    xor_drop_reduce(&mut oracle, 97 - 66, 0);
    let rep = RepetitionCode::new(3);
    let code = ConcatenatedCode::new(vec![
        &BogosrndCode19_6,
        &BogosrndCode19_6,
        &rep,
        &WagnerCode25_15,
    ]);
    code_reduce(&mut oracle, &code);

    let secret = oracle.secret.as_binvector(code.dimension());
    let solution = fwht_solve(oracle);
    log::info!("done");

    println!("Found:  {:?}", solution);
    println!("Actual: {:?}", secret);
}

#[cfg(not(all(feature = "wagner_25", feature = "bogosrnd_19")))]
fn main() {
    println!("necessary feature disabled");
}
