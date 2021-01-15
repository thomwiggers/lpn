//! Implements the covering codes reduction and sparse secret transformation
use crate::oracle::{LpnOracle, Sample};
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
    let k = oracle.get_k();
    let mut rng = thread_rng();
    // get M, e, c'
    let (m, c_prime, samples) = loop {
        let (a, b, samples) = {
            // FIXME maybe cheat by picking from the first million?
            let samples: Vec<_> = oracle
                .samples
                .choose_multiple(&mut rng, k)
                .cloned()
                .collect();
            // replace by matrix directly?
            let mut b = BinVector::with_capacity(k);
            //let mut e = BinVector::with_capacity(k);
            (
                // vectors on the columns
                BinMatrix::new(
                    samples
                        .iter()
                        .map(|q| {
                            b.push(q.get_product());
                            //e.push(q.e);
                            q.as_binvector(k)
                        })
                        .collect(),
                ),
                b,
                //e,
                samples,
            )
        };
        if a.clone().echelonize() == k {
            break (a, b, samples);
        }
    };

    // update the secret:
    let original_secret = oracle.secret.as_binvector(k);
    println!(
        "the secret prior to reduction to a sparse secret was: {:?}",
        original_secret
    );
    if oracle.delta == 1.0 {
        debug_assert_eq!((&original_secret * &m.transposed()), c_prime, "this one fails if tau > 0");
    }
    let new_secret = &(&m * &original_secret) + &c_prime;
    oracle.secret = Sample::from_binvector(&new_secret, false);

    debug_assert_eq!(
        (&new_secret + &c_prime) * m.transposed().inverted(),
        original_secret
    );

    let m_t_inv = m.inverted();

    // remove the samples we took
    // When https://github.com/rust-lang/rust/issues/40062 is stabilized,
    // this might be possible in a more elegant way.
    for q in samples.into_iter() {
        let pos = oracle.samples.iter().position(|item| item == &q).unwrap();
        oracle.samples.swap_remove(pos);
    }

    // update the samples
    //let secret = &oracle.secret;
    oracle.samples.par_iter_mut().for_each(|query| {
        let new_v = &query.as_binvector(k) * &m_t_inv;
        let new_product = query.get_product() ^ &new_v * &c_prime;
        *query = Sample::from_binvector(&new_v, new_product);
        //debug_assert_eq!((secret * &new_v) ^ query.e, query.c);
    });

    oracle.sparse_transform_matrix = Some(m);
    oracle.sparse_transform_vector = Some(c_prime);
    oracle.delta_s = oracle.delta;
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
        oracle.delta_s > 0.0,
        "This reduction only works for sparse secrets!"
    );
    assert_eq!(
        oracle.get_k() as usize,
        code.length(),
        "The length of the code does not match the problem size!"
    );

    println!("Decoding samples");
    let k = oracle.get_k();
    oracle.samples.par_iter_mut().for_each(|query| {
        let new_sample = code.decode_to_message(&query.as_binvector(k)).expect("Couldn't decode??");
        *query = Sample::from_binvector(&new_sample, query.get_product())
    });

    println!("Note that we transformed the secret $s$ into $s'=s*G^T$!");
    let gen_t = code.generator_matrix().transposed();
    oracle.secret = Sample::from_binvector(&(&oracle.secret.as_binvector(k) * &gen_t), false);

    oracle.truncate( code.dimension());

    println!("Computing new delta");
    oracle.delta *= code.bias(oracle.delta_s);
    println!("New delta = {}", oracle.delta);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::codes::HammingCode15_11;
    use crate::lf1::fwht_solve;

    #[test]
    fn test_unsparse() {
        // setup
        let mut oracle: LpnOracle = LpnOracle::new(15, 1.0 / 4.0);
        oracle.secret = Sample::from_binvector(&BinVector::from_function(15, |x| x % 2 == 0), false);
        let secret = oracle.secret.as_binvector(oracle.get_k());
        oracle.get_samples(1000);

        // check the sparse secret reduction
        sparse_secret_reduce(&mut oracle);
        let unsps = unsparse_secret(&oracle, &oracle.secret.as_binvector(oracle.get_k()));
        assert_eq!(secret, unsps, "sparse/unsparse unequal");
    }

    #[test]
    fn test_reduction() {
        // setup
        let mut oracle: LpnOracle = LpnOracle::new(15, 0.0 / 8.0);
        oracle.secret = Sample::from_binvector(&BinVector::from_function(15, |x| x % 2 == 0), false);
        oracle.get_samples(1_000_000);

        // check the sparse secret reduction
        sparse_secret_reduce(&mut oracle);

        // do the reduction
        let code = HammingCode15_11;
        code_reduce(&mut oracle, &code);
        // get transformed secret for checking
        let secret = oracle.secret.as_binvector(oracle.get_k());
        // solve
        let fwht_solution = fwht_solve(oracle.clone());
        assert_eq!(secret, fwht_solution, "Found wrong solution");
    }
}
