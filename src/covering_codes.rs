//! Implements the covering codes reduction and sparse secret transformation
use std::sync::{Arc, Mutex};

use crate::{
    oracle::{LpnOracle, Sample},
    random::lpn_thread_rng,
};
use indicatif::ProgressBar;
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
    let k = oracle.get_k();
    let mut rng = lpn_thread_rng();

    // cheat by picking from the first million
    let searchspace = std::cmp::min(oracle.samples.len(), 1_000_000);

    // get M, e, c'
    let (m, c_prime, samples) = loop {
        let (a, b, samples) = {
            let samples: Vec<_> = oracle.samples[..searchspace]
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
    log::debug!(
        "the secret prior to reduction to a sparse secret was: {:?}",
        original_secret
    );

    if oracle.delta == 1.0 {
        debug_assert_eq!(
            (&original_secret * &m.transposed()),
            c_prime,
            "this one fails if tau > 0"
        );
    }
    let new_secret = &(&m * &original_secret) + &c_prime;
    oracle.secret = Sample::from_binvector(&new_secret, false);

    debug_assert_eq!(
        (&new_secret + &c_prime) * m.transposed().inverted(),
        original_secret
    );

    let m_t_inv = m.inverted();

    log::trace!("removing the samples we took for the transformation matrix");
    let mut poses = samples
        .into_par_iter()
        .map(|sample| {
            oracle.samples[..searchspace]
                .iter()
                .position(|item| item == &sample)
                .unwrap()
        })
        .collect::<Vec<_>>();
    poses.sort_unstable();
    poses.into_iter().rev().for_each(|pos| {
        oracle.samples.swap_remove(pos);
    });

    // update the samples
    //let secret = &oracle.secret;
    let m_t_inv_t = &m_t_inv.transposed();

    let progress = ProgressBar::new(oracle.samples.len() as u64);
    log::info!("Sparse-secretifying samples");
    progress.set_draw_delta(oracle.samples.len() as u64 / 100);
    progress.reset();
    let progress = Arc::new(Mutex::new(progress));
    oracle.samples.par_chunks_mut(10000).for_each(|queries| {
        let len_chunk = queries.len();
        for query in queries {
            let new_v = m_t_inv_t.mul_slice(query.get_sample()).as_vector();
            let new_product = query.get_product() ^ &new_v * &c_prime;
            *query = Sample::from_binvector(&new_v, new_product);
            //debug_assert_eq!((secret * &new_v) ^ query.e, query.c);
        }
        progress.lock().unwrap().inc(len_chunk as u64);
    });
    progress.lock().unwrap().finish_and_clear();

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

    log::info!("Decoding samples");
    let progress = ProgressBar::new(oracle.samples.len() as u64);
    progress.set_draw_delta(oracle.samples.len() as u64 / 100);
    progress.reset();
    let progress = Arc::new(Mutex::new(progress));
    oracle.samples.par_chunks_mut(10000).for_each(|queries| {
        let chunk_len = queries.len();
        for query in queries {
            code.decode_sample(query)
        }
        progress.lock().unwrap().inc(chunk_len as u64);
    });
    progress.lock().unwrap().finish_and_clear();

    log::warn!(
        "Note that we transformed the secret $s$ into $s'=s*G^T$ with k' = {}!",
        oracle.get_k()
    );
    let k = oracle.get_k();
    let gen_t = code.generator_matrix().transposed();
    oracle.secret = Sample::from_binvector(&(&oracle.secret.as_binvector(k) * &gen_t), false);

    unsafe { oracle.set_k(code.dimension()) };

    //log::trace!("Computing new delta");
    //oracle.delta *= code.bias(oracle.delta_s);
    //log::debug!("New delta = {}", oracle.delta);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unsparse() {
        // setup
        let mut oracle: LpnOracle = LpnOracle::new(15, 1.0 / 4.0);
        oracle.secret =
            Sample::from_binvector(&BinVector::from_function(15, |x| x % 2 == 0), false);
        let secret = oracle.secret.as_binvector(oracle.get_k());
        oracle.get_samples(1000);

        // check the sparse secret reduction
        sparse_secret_reduce(&mut oracle);
        let unsps = unsparse_secret(&oracle, &oracle.secret.as_binvector(oracle.get_k()));
        assert_eq!(secret, unsps, "sparse/unsparse unequal");
    }

    #[cfg(feature = "hamming")]
    #[test]
    fn test_reduction() {
        use crate::codes::HammingCode15_11;
        use crate::lf1::fwht_solve;

        // setup
        let mut oracle: LpnOracle = LpnOracle::new(15, 0.0 / 8.0);
        oracle.secret =
            Sample::from_binvector(&BinVector::from_function(15, |x| x % 2 == 0), false);
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
