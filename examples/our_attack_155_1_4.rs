extern crate lpn;

// [1, 155, 46.450923007920466, 39.59516467811528, 32.800748811765175, 32.40037440588259, 
// 'drop_reduce (c= 1, k=155); 1,142,41.5957014763019,39.59516467811528,32.800748811765175,32.40037440588259,
// xor_drop_reduce (c=1, k=142, opt_k-b1=111), 2,111,41.03994268217354,39.59516467811528,32.800748811765175,32.800748811765175,
// xor_drop_reduce (c=2, k=111, opt_k-b1=79), 4,79,40.379425177621826,38.905278371707446,32.60149762353034,32.60149762353034,
// xor_drop_reduce (c=4, k=79, opt_k-b1=47), 8,47,39.735690258577044,37.75758409873832,32.202995247060684,32.202995247060684,
// codes_reduce + WHT (c=8, k=47, opt_k_prime=30, code=[47, 30] concatenated code with from ([27,17] Wagner, [20,13] Wagner))']
    

#[cfg(all(feature = "wagner_27", feature="guava_20"))]
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

    let k = 155;
    let tau = 1.0 / 4.0;

    let mut oracle: LpnOracle = LpnOracle::new(k, tau);
    let start_len = 5668714268;
    let k = k as usize;
    oracle.get_samples_drop(start_len + 1000,  k-142);
    log::info!("Collected samples.");
    sparse_secret_reduce(&mut oracle);
    xor_drop_reduce(&mut oracle, 142 - 111, 0);
    xor_drop_reduce(&mut oracle, 111 - 79, 0);
    xor_drop_reduce(&mut oracle, 79 - 47, 0);
    let code = ConcatenatedCode::new(vec![&WagnerCode27_17, &GuavaCode20_13]);
    code_reduce(&mut oracle, &code);

    let secret = oracle.secret.as_binvector(code.dimension());
    let solution = fwht_solve(oracle);
    log::info!("done");

    println!("Found:  {:?}", solution);
    println!("Actual: {:?}", secret);
}

#[cfg(not(all(feature = "wagner_27", feature = "guava_20")))]
fn main() {
    println!("necessary feature disabled");
}
