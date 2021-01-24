//! Defines the Pooled Gauss solving algorithms by Esser, Kübler and May
use crate::oracle::LpnOracle;
use m4ri_rust::friendly::solve_left;
use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;
use rand::prelude::*;
use rayon::prelude::*;

use std::{
    cell::RefCell,
    sync::{Arc, Mutex},
};

/// Solves an LPN problem using Pooled Gauss
#[allow(clippy::many_single_char_names, clippy::needless_pass_by_value)]
pub fn pooled_gauss_solve(oracle: LpnOracle) -> BinVector {
    let mut rng = thread_rng();

    let k = oracle.get_k();
    let alpha = 0.5f64.powi(k as i32);
    let tau = (1.0 - oracle.delta) / 2.0;
    let beta = ((1f64 - tau) / 2f64).powi(k as i32);
    let m: f64 = (((1.5 * (1.0 / alpha).ln()).sqrt() + (1.0 / beta).ln().sqrt()) / (0.5 - tau))
        .powi(2)
        .floor();
    let c = (tau * m + (3.0 * (0.5 - tau) * (1.0 / alpha).ln() * m).sqrt().floor()) as u32;
    let m = m as usize;

    log::info!(
        "Attempting Pooled Gauss solving method, k={}, tau={}",
        k, tau
    );
    log::trace!("Target secret weight <= {}", c);
    log::trace!("Building (Am, b) with length {}", m);
    let (am, bm) = sample_matrix(m, &oracle, &mut rng);
    debug_assert_eq!(am.ncols(), k);
    debug_assert_eq!(am.nrows(), m);
    debug_assert_eq!(bm.nrows(), m);
    debug_assert_eq!(bm.ncols(), 1);

    let secret = &oracle.secret.as_binvector(k);

    let test = |s_prime: &BinMatrix| {
        debug_assert_eq!(s_prime.nrows(), k);
        debug_assert_eq!(s_prime.ncols(), 1);

        let mut testproduct = &am * s_prime;
        testproduct += &bm;
        let result = testproduct.count_ones() <= c;
        debug_assert_eq!(
            result,
            &s_prime.as_vector() == secret,
            "Test will reject or accept an (in)correct secret with weight {} <= {}",
            testproduct.count_ones(),
            c
        );
        result
    };

    log::debug!("Starting random sampling of invertible (A, b)");

    let s_prime_finder = move |(sender, rng): &mut (Arc<Mutex<Option<BinMatrix>>>, _), _| {
        for _ in 0..10000 {
            // find k-rank matrix
            let (a, mut b) = loop {
                let (a_try, b_try) = sample_matrix(k as usize, &oracle, rng);
                // TODO is this check necessary?
                // TODO avoid allocate?
                if a_try.clone().echelonize() == k as usize {
                    break (a_try, b_try);
                }
            };
            // A*s = b
            if !solve_left(a, &mut b) {
                log::warn!("Somehow, solving failed....");
                continue;
            }
            let result = { test(&b) };
            if result {
                println!("Found {:?}!", b.as_vector());
                let mut sender = sender.lock().unwrap();
                sender.replace(b);
                break;
            }
        }

        if sender.lock().unwrap().is_none() {
            Some(())
        } else {
            None
        }
    };

    let sender_parent = Arc::new(Mutex::new(None));
    let sender = sender_parent.clone();

    rayon::iter::repeat(())
        .try_for_each_init(|| (sender.clone(), rand::thread_rng()), s_prime_finder);

    let sender = sender_parent.lock().unwrap();
    let s_prime = sender.as_ref().unwrap();

    s_prime.as_vector()
}

/// Randomly sample ``k`` queries from the oracle as a ``(A, s)``.
fn sample_matrix<'a>(k: usize, oracle: &LpnOracle, rng: &mut ThreadRng) -> (BinMatrix, BinMatrix) {
    thread_local!(static TLS: RefCell<(Vec<&'static [u64]>, BinVector)> = RefCell::new((Vec::new(), BinVector::new())));

    TLS.with(|stor| {
        let mut stor = stor.borrow_mut();
        let (slices, b_bits) = &mut (*stor);
        let samples = oracle.samples.choose_multiple(rng, k);
        slices.extend(samples.map(|q| {
            b_bits.push(q.get_product());
            // this is okay, because we clear out samples at the end.
            unsafe { std::mem::transmute::<&'_ [u64], &'static [u64]>(q.get_sample()) }
        }));
        // replace by matrix directly?
        let mat = BinMatrix::from_slices(slices, oracle.get_k());
        let ret_b = b_bits.as_column_matrix();
        slices.clear();
        b_bits.clear();
        (mat, ret_b)
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_gauss() {
        let mut oracle: LpnOracle = LpnOracle::new(32, 1.0 / 4.0);
        oracle.get_samples(4000555);
        let secret = oracle.secret.clone();
        let solution = pooled_gauss_solve(oracle);
        assert_eq!(solution, secret.as_binvector(32));
    }
}
