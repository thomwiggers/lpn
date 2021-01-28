extern crate lpn;

// [1, 122, 32.429681784686686, 30.806759398144838, 23.924116348782995, 23.924116348782995,
// 'drop_reduce (c= 1, k=122); 1,118,32.30539104034155,30.80675939814483 8,23.924116348782995,23.924116348782995,
// xor_drop_reduce (c=1, k=118, opt_k-b1=95), 2,95,31.67524523784735,30.418088305896934,23.848232697565987,23.84823269756 5987,
// xor_drop_reduce (c=2, k=95, opt_k-b1=72), 4,72,30.893424755099776,29.866390396574285,23.696465395131973,23.696465395131973,
// xor_drop_reduce (c=4, k=72, op t_k-b1=49), 8,49,29.919961826745052,29.007640634379154,23.392930790263947,23.392930790263947,
// codes_reduce + WHT (c=8, k=49, opt_k_prime=20, code=[49, 20] concatenated code with from ([25,15] Wagner, [19,4] guava-3.15, [5,1] repetition))']

#[cfg(all(feature = "wagner", feature = "guava"))]
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

    let k = 122;
    let tau = 1.0 / 8.0;

    let mut oracle: LpnOracle = LpnOracle::new(k, tau);
    let start_len = 2usize.pow(24);
    let k = k as usize;
    oracle.get_samples_drop(start_len + 1000, k - 118);
    assert_eq!(oracle.get_k(), 118);
    log::info!("Collected samples.");
    sparse_secret_reduce(&mut oracle);
    xor_drop_reduce(&mut oracle, 118 - 95, 0);
    xor_drop_reduce(&mut oracle, 95 - 72, 0);
    xor_drop_reduce(&mut oracle, 72 - 49, 0);
    let rep = &RepetitionCode::new(5);
    let code = ConcatenatedCode::new(vec![&WagnerCode25_15, &GuavaCode19_4, rep]);
    code_reduce(&mut oracle, &code);

    let secret = oracle.secret.as_binvector(code.dimension());
    let solution = fwht_solve(oracle);
    log::info!("done");

    println!("Found:  {:?}", solution);
    println!("Actual: {:?}", secret);
}

#[cfg(any(not(feature = "wagner"), not(feature = "guava")))]
fn main() {
    println!("necessary feature disabled");
}
