extern crate lpn;

// [1, 140, 43.02291022151072, 37.917015857494334, 30.905788602071084, 30.905788602071084,
// 'drop_reduce (c= 1, k=140); 1,129,39.35036434401339,37.917015857494334,30.905788602071084,30.905788602071084,
// xor_drop_reduce (c=1, k=129, opt_k-b1=99), 2,99,38.683168504236804,37.44093382422178,30.811577204142168,30.811577204142168,
// xor_drop_reduce (c=2, k=99, opt_k-b1=69), 4,69,37.8905182117595,36.731678865062506,30.623154408284332,30.623154408284332,
// xor_drop_reduce (c=4, k=69, opt_k-b1=39), 8,39,37.03359103895924,35.53171103543092,30.246308816568664,30.246308816568664,
// codes_reduce code=[39, 27] concatenated code with from ([31,20] Wagner, [2,1] Punctured Hamming, [1,1] trivial, [1,1] trivial, [1,1] trivial, [1,1] trivial, [1,1] trivial, [1,1] trivial))']
// + WHT (c=8, k=39, opt_k_prime=27,

#[cfg(all(feature = "wagner_31"))]
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

    let k = 140;
    let tau = 1.0 / 4.0;

    let mut oracle: LpnOracle = LpnOracle::new(k, tau);
    let start_len = 2011728687;
    let k = k as usize;
    oracle.get_samples_drop(start_len + 1000, k - 129);
    log::info!("Collected samples.");
    sparse_secret_reduce(&mut oracle);
    xor_drop_reduce(&mut oracle, 129 - 99, 0);
    xor_drop_reduce(&mut oracle, 99 - 69, 0);
    xor_drop_reduce(&mut oracle, 69 - 69, 0);

    let rep = RepetitionCode::new(2);
    let id = IdentityCode::new(1);
    let code = ConcatenatedCode::new(vec![&WagnerCode31_20, &rep, &id, &id, &id, &id, &id, &id]);
    code_reduce(&mut oracle, &code);

    let secret = oracle.secret.as_binvector(code.dimension());
    let solution = fwht_solve(oracle);
    log::info!("done");

    println!("Found:  {:?}", solution);
    println!("Actual: {:?}", secret);
}

#[cfg(not(all(feature = "wagner_31")))]
fn main() {
    println!("necessary feature disabled");
}
