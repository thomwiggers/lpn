use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;
use oracle::LpnOracle;
use rayon::prelude::*;

use codes::BinaryCode;
use rand;

/// Sparse secret reduction
///
/// Changes the distribution of the secret to that of the noise
///
/// $k' = k$
/// $n' = n-k$
/// $d' = d$
/// $d'_s = d$
pub fn reduce_sparse_secret(mut oracle: LpnOracle) -> LpnOracle {
    println!("Reducing to a sparse secret");
    let k = oracle.k;
    let mut rng = rand::thread_rng();
    // get M, c'
    let (m, c_prime, queries) = loop {
        let (mut a, b, queries) = {
            let queries =
                rand::seq::sample_iter(&mut rng, oracle.queries.iter().cloned(), k as usize)
                    .unwrap();
            // replace by matrix directly?
            let mut b = BinVector::with_capacity(k as usize);
            (
                // vectors on the columns
                BinMatrix::new(
                    queries
                        .iter()
                        .map(|q| {
                            b.push(q.s);
                            q.a.clone()
                        })
                        .collect(),
                ),
                b,
                queries,
            )
        };
        if a.echelonize() == k as usize {
            break (a, b, queries);
        }
    };

    // update the secret:
    println!(
        "the secret prior to reduction to a sparse secret was: {:?}",
        oracle.secret
    );
    oracle.secret = &(&m * &oracle.secret) + &c_prime;

    let m_t_inv = m.inverted();
    // update the queries
    let secret = &oracle.secret.clone();
    oracle.queries = oracle
        .queries
        .into_par_iter()
        // remove the queries we took
        .filter(|q| !queries.contains(q))
        .map(|mut query| {
            let new_v = &query.a * &m_t_inv;
            query.s ^= &new_v * &c_prime;
            debug_assert_eq!(secret * &new_v ^ query.e, query.s);
            query.a = new_v;
            query
        })
        .collect();

    oracle.delta_s = oracle.delta;

    oracle
}

/// Reduce using the covering codes attack (Guo, Johansson, Lohndal; 2014)
///
/// $k' = dim(G)$
/// $n' = n$
/// $d' = d * bc$
/// $d'_s$ depends on $d_s$ and $G$.
pub fn code_reduction<'a, T: BinaryCode<'a> + Sync>(
    mut oracle: LpnOracle,
    code: T,
) -> LpnOracle {
    assert_eq!(
        oracle.k as usize,
        code.length(),
        "The length of the code does not match the problem size!"
    );

    println!("Decoding queries");
    oracle.queries.par_iter_mut().for_each(|query| {
        (*query).a = code.decode_to_message(&query.a);
        debug_assert_eq!(query.a.len(), code.dimension());
    });

    println!("Note that we transformed the secret $s$ into $s'=s*G^T$!");
    let gen_t = code.generator_matrix().transposed();
    oracle.secret = &oracle.secret * &gen_t;

    debug_assert_eq!(oracle.secret.len(), code.dimension());
    oracle.k = code.dimension() as u32;

    println!("Computing new delta");
    oracle.delta = oracle.delta * code.bias(oracle.delta_s);
    println!("New delta = {}", oracle.delta);

    oracle
}
