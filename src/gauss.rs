use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::solve_left;
use m4ri_rust::friendly::BinVector;
use oracle::LpnOracle;
use rand;

pub fn pooled_gauss_solve(oracle: LpnOracle) -> BinVector {
    println!("Attempting Pooled Gauss solving method");
    let k = oracle.k;
    let alpha = 0.5f32.powi(k as i32);
   let beta = ((1f32 - oracle.tau) / 2f32).powi(k as i32);
    let m: f32 = (((1.5 * (1.0 / alpha).ln()).sqrt() + (1.0 / beta).ln().sqrt())
        / (0.5 - oracle.tau))
        .powi(2)
        .floor();
    let c = (oracle.tau * m
        + (3.0 * (0.5 - oracle.tau) * (1.0 / alpha).ln() * m)
            .sqrt()
            .floor()) as u32;
    let m = m as usize;
    let mut rng = rand::thread_rng();

    println!("Building (Am, b) with length {}", m);
    let (am, bm) = {
        let queries = rand::seq::sample_iter(&mut rng, oracle.queries.iter(), m).unwrap();
        let mut b = BinVector::with_capacity(m);
        (
            BinMatrix::new(
                queries
                    .into_iter()
                    .map(|q| {
                        b.push(q.s);
                        q.a.clone()
                    })
                    .collect(),
            ),
            b.as_column_matrix(),
        )
    };

    let mut tries = 0;
    let mut test = |s_prime: &BinMatrix| {
        tries += 1;
        if tries % 1000 == 0 {
            println!("Attempt {}...", tries);
        }
        let mut testproduct = &am * s_prime;
        testproduct += &bm;
        testproduct.as_vector().count_ones() <= c
    };

    println!("Starting random sampling of invertible (A, b)");
    let s_prime = loop {
        let (a, b) = loop {
            let (mut a, b) = {
                let queries =
                    rand::seq::sample_iter(&mut rng, oracle.queries.iter(), k as usize).unwrap();
                // replace by matrix directly?
                let mut b = BinVector::with_capacity(k as usize);
                (
                    BinMatrix::new(
                        queries
                            .into_iter()
                            .map(|q| {
                                b.push(q.s);
                                q.a.clone()
                            })
                            .collect(),
                    ),
                    b,
                )
            };
            if a.echelonize() == k as usize {
                break (a, b);
            }
        };

        // A*s = b
        let mut b = b.as_column_matrix();
        if !solve_left(a, &mut b) {
            println!("Somehow, solving failed....");
            continue;
        }
        if test(&b) {
            break b;
        }
    };

    s_prime.as_vector()
}
