use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;
use oracle::LpnOracle;
use rayon::prelude::*;

use codes::BinaryCode;
use rand;

pub fn reduce_sparse_secret(mut oracle: LpnOracle) -> LpnOracle {
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
            // remove the queries we took
            break (a, b, queries);
        }
    };

    let m_t = m.transpose();
    let m_t_inv = m_t.inverted();

    // update the secret:
    println!(
        "the secret prior to reduction to a sparse secret was: {:?}",
        oracle.secret
    );
    oracle.secret = &(m_t * oracle.secret) + &c_prime;

    // update the queries
    oracle.queries = oracle
        .queries
        .into_par_iter()
        .filter(|q| !queries.contains(q))
        .map(|mut query| {
            query.a = &query.a * &m_t_inv;
            query.s = &query.a * &c_prime;
            query
        })
        .collect();

    oracle
}

/// Reduce using the covering codes attack (Guo, Johansson, Lohndal; 2014)
pub fn reduce_covering_codes<'a, T: BinaryCode<'a> + Sync>(
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
    println!("Note that we decoded the secret!");
    oracle.secret = code.decode_to_message(&oracle.secret);
    debug_assert_eq!(oracle.secret.len(), code.dimension());
    oracle.k = code.dimension() as u32;

    oracle
}
