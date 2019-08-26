//! Implements the covering codes reduction and sparse secret transformation
use crate::oracle::LpnOracle;
use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;
use rayon::prelude::*;

use crate::codes::BinaryCode;
use rand::prelude::*;

/// Sparse secret reduction
///
/// Changes the distribution of the secret to that of the noise
///
/// `$k' = k$`
/// `$n' = n-k$`
/// `$d' = d$`
/// `$d'_s = d$`
pub fn sparse_secret_reduce(oracle: &mut LpnOracle) {
    println!("Reducing to a sparse secret");
    let k = oracle.k;
    let mut rng = thread_rng();
    // get M, e, c'
    let (m, c_prime, e, samples) = loop {
        let (a, b, e, samples) = {
            let samples: Vec<_> = oracle
                .samples
                .choose_multiple(&mut rng, k as usize)
                .cloned()
                .collect();
            // replace by matrix directly?
            let mut b = BinVector::with_capacity(k as usize);
            let mut e = BinVector::with_capacity(k as usize);
            (
                // vectors on the columns
                BinMatrix::new(
                    samples
                        .iter()
                        .map(|q| {
                            b.push(q.c);
                            e.push(q.e);
                            q.a.clone()
                        })
                        .collect(),
                ),
                b,
                e,
                samples,
            )
        };
        if a.clone().echelonize() == k as usize {
            break (a, b, e, samples);
        }
    };

    // update the secret:
    let original_secret = oracle.secret.clone();
    println!(
        "the secret prior to reduction to a sparse secret was: {:?}",
        original_secret
    );
    debug_assert_eq!((&original_secret * &m.transposed()) + e, c_prime);
    oracle.secret = &(&m * &original_secret) + &c_prime;

    debug_assert_eq!(
        (&oracle.secret + &c_prime) * m.transposed().inverted(),
        original_secret
    );

    let m_t_inv = m.inverted();
    // update the samples
    let secret = &oracle.secret.clone();

    // remove the samples we took
    // XXX do this differently so we don't need unstable features.
    for q in samples.into_iter() {
        oracle.samples.remove_item(&q);
    }

    oracle
        .samples
        .par_iter_mut()
        .for_each(|query| {
            let new_v = &query.a * &m_t_inv;
            query.c ^= &new_v * &c_prime;
            debug_assert_eq!((secret * &new_v) ^ query.e, query.c);
            query.a = new_v;
        });

    oracle.sparse_transform_matrix = Some(m);
    oracle.sparse_transform_vector = Some(c_prime);
    oracle.delta_s = oracle.delta;
    oracle.secret.truncate(oracle.k as usize);
}

/// Undo the sparse secret reduction for secrets.
pub fn unsparse_secret(oracle: &LpnOracle, secret: &BinVector) -> BinVector {
    let m = &oracle
        .sparse_transform_matrix
        .clone()
        .expect("Not a sparse oracle");
    let c_prime = &oracle.sparse_transform_vector.clone().unwrap();

    (secret + c_prime) * m.transposed().inverted()
}

/// Reduce using the covering codes attack (Guo, Johansson, Lohndal; 2014)
///
/// $k' = dim(G)$
/// $n' = n$
/// $d' = d * bc$
/// $d'_s$ depends on $d_s$ and $G$.
pub fn code_reduce<T: BinaryCode + Sync>(oracle: &mut LpnOracle, code: &T) {
    assert!(
        oracle.delta_s < std::f64::EPSILON,
        "This reduction only works for sparse secrets!"
    );
    assert_eq!(
        oracle.k as usize,
        code.length(),
        "The length of the code does not match the problem size!"
    );

    println!("Decoding samples");
    oracle.samples.par_iter_mut().for_each(|query| {
        (*query).a = code.decode_to_message(&query.a).expect("Couldn't decode??");
        debug_assert_eq!(query.a.len(), code.dimension());
    });

    println!("Note that we transformed the secret $s$ into $s'=s*G^T$!");
    let gen_t = code.generator_matrix().transposed();
    oracle.secret = &oracle.secret * &gen_t;

    debug_assert_eq!(oracle.secret.len(), code.dimension());
    oracle.k = code.dimension() as u32;

    println!("Computing new delta");
    oracle.delta *= code.bias(oracle.delta_s);
    println!("New delta = {}", oracle.delta);
}
