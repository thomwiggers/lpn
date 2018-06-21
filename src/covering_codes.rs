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
    // get M, e, c'
    let (m, c_prime, e, queries) = loop {
        let (a, b, e, queries) = {
            let queries =
                rand::seq::sample_iter(&mut rng, oracle.queries.iter().cloned(), k as usize)
                    .unwrap();
            // replace by matrix directly?
            let mut b = BinVector::with_capacity(k as usize);
            let mut e = BinVector::with_capacity(k as usize);
            (
                // vectors on the columns
                BinMatrix::new(
                    queries
                        .iter()
                        .map(|q| {
                            b.push(q.s);
                            e.push(q.e);
                            q.a.clone()
                        })
                        .collect(),
                ),
                b,
                e,
                queries,
            )
        };
        if a.clone().echelonize() == k as usize {
            break (a, b, e, queries);
        }
    };

    // update the secret:
    let original_secret = oracle.secret;
    println!(
        "the secret prior to reduction to a sparse secret was: {:?}",
        original_secret
    );
    debug_assert_eq!((&original_secret * &m.transposed()) + e, c_prime);
    oracle.secret = &(&m * &original_secret) + &c_prime;

    debug_assert_eq!((&oracle.secret + &c_prime) * m.transposed().inverted(),
                     original_secret);

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

    oracle.sparse_transform_matrix = Some(m);
    oracle.sparse_transform_vector = Some(c_prime);
    oracle.delta_s = oracle.delta;
    oracle.secret.truncate(oracle.k as usize);

    oracle
}

/// Undo the sparse secret reduction for secrets.
pub fn unsparse_secret(oracle: &LpnOracle, secret: &BinVector) -> BinVector{
    let m = &oracle.sparse_transform_matrix.clone().expect("Not a sparse oracle");
    let c_prime = &oracle.sparse_transform_vector.clone().unwrap();

    (secret + c_prime) * m.transposed().inverted()
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
    assert_ne!(oracle.delta_s, 0.0,
               "This reduction only works for sparse secrets!");
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
