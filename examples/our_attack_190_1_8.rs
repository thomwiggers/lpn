extern crate lpn;

/// [1, 190, 40.923267842485785, 38.52058775592134, 31.07820668219669, 31.004887917637294, 
/// 'drop_reduce (c= 1, k=190); 1,183,40.484012895515356,38.52058775592134, 31.07820668219669,31.004887917637294,
/// xor_drop_reduce (c=1, k=183, opt_k-b1=153), 2,153,40.0548853672661,38.26716367796724,31.07820668219669,31.00977583527459,
/// xor_drop_reduce (c=2, k=153, opt_k-b1=123), 4,123,39.55752497487358,37.96206617588841,31.07820668219669,31.019551670549173,
/// xor_drop_reduce (c=4, k=123, opt_k-b1=93), 8,93,38.96801338455373,37.57826215220638,31.07820668219669,31.039103341098347,
/// xor_drop_reduce (c=8, k=93, opt_k-b1=63), 16,63,38.24988153482742,37.05548660569661,31.07820668219669,31.07820668219669,
/// codes_reduce  code=[63, 28] concatenated code with from ([25,15] Wagner, [19,7] rnd150926, [19,6] rnd150926))
/// WHT (c=16, k=63, opt_k_prime=28,']
    

#[cfg(all(feature = "wagner_25", feature="bogosrnd_19"))]
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

    let k = 190;
    let tau = 1.0 / 8.0;

    let mut oracle: LpnOracle = LpnOracle::new(k, tau);
    let start_len = 2154771762;
    let k = k as usize;
    oracle.get_samples_drop(start_len + 1000,  k-183);
    log::info!("Collected samples.");
    sparse_secret_reduce(&mut oracle);
    xor_drop_reduce(&mut oracle, 183 - 153, 0);
    xor_drop_reduce(&mut oracle, 153 - 123, 0);
    xor_drop_reduce(&mut oracle, 123 - 93, 0);
    xor_drop_reduce(&mut oracle, 93 - 63, 0);
    let code = ConcatenatedCode::new(vec![&WagnerCode25_15, &BogosrndCode19_7, &BogosrndCode19_6]);
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
