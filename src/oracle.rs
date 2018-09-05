//! Describes the LPN problem oracle on which we apply reductions and solving algorithms
use m4ri_rust::friendly::*;
use rand;
use rand::distributions::{Bernoulli, Distribution};
use std::ops::Range;

use rayon::prelude::*;

/// Represents a sample in the oracle
///
/// `<a, s> + e = c`
#[derive(Debug, Clone, PartialEq)]
pub struct Sample {
    pub a: BinVector,
    pub c: bool,
    pub e: bool,
}

impl Sample {
    /// Get the Hamming weight of the sample
    pub fn count_ones(&self) -> u32 {
        self.a.count_ones()
    }
}

/// This struct represents the oracle of the LPN problem.
///
/// We need to obtain the queries needed before applying reductions or transformations.
#[derive(Clone)]
pub struct LpnOracle {
    pub samples: Vec<Sample>,
    pub secret: BinVector,
    pub k: u32,
    pub delta: f64,
    pub delta_s: f64,
    pub(crate) sparse_transform_matrix: Option<BinMatrix>,
    pub(crate) sparse_transform_vector: Option<BinVector>,
}

impl LpnOracle {
    /// Create a new LPN problem with a random secret
    pub fn new(k: u32, tau: f64) -> LpnOracle {
        debug_assert!(0.0 <= tau && tau < 1.0, "0 <= tau < 1");
        debug_assert!(k > 0, "k > 0");
        let secret = BinVector::random(k as usize);
        println!("Constructed Oracle with k={}, τ={:0.5}", k, tau);

        LpnOracle {
            samples: vec![],
            secret,
            k,
            delta: 1f64 - 2f64 * tau,
            delta_s: 0f64, // uniformly random
            sparse_transform_matrix: None,
            sparse_transform_vector: None,
        }
    }

    /// Create a new LPN problem with a set secret
    pub fn new_with_secret(secret: BinVector, tau: f64) -> LpnOracle {
        let mut lpn = Self::new(secret.len() as u32, tau);
        lpn.secret = secret;
        lpn
    }

    /// Get new samples from the oracle
    ///
    /// These samples are stored in ``oracle.samples``
    ///
    /// Uses parallelism
    pub fn get_samples(&mut self, n: usize) {
        println!("Getting {} samples", n);
        self.samples.reserve(n);
        let k = self.k as usize;
        let len = if k % 8 == 0 { k / 8 } else { k / 8 + 1 };
        debug_assert!(len > 0);

        let tau = (1.0 - self.delta) / 2.0;
        let dist = Bernoulli::new(tau);
        let secret = self.secret.clone();
        self.samples.par_extend((0..n).into_par_iter().map(|_| {
            let mut rng = rand::thread_rng();
            let vector = BinVector::random(k);
            debug_assert_eq!(vector.len(), k);
            let e = dist.sample(&mut rng);

            Sample {
                c: &secret * &vector ^ e,
                a: vector,
                e,
            }
        }));
    }
}

pub(crate) fn query_bits_range(v: &BinVector, range: &Range<usize>) -> u64 {
    // FIXME speed up
    let len = range.end - range.start;
    debug_assert!(len < 64, "Needs to fit in u64");
    let mut result = 0u64;
    for (i, ri) in (range.clone()).rev().enumerate() {
        result ^= (v[ri] as u64) << i;
    }
    debug_assert_eq!(result >> len, 0);
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bitrange() {
        let v = BinVector::from_bytes(&[0b10011101u8]);
        assert_eq!(query_bits_range(&v, &(0..3)), 0b100);
        assert_eq!(query_bits_range(&v, &(2..4)), 0b01);
        assert_eq!(query_bits_range(&v, &(3..4)), 0b1);
        assert_eq!(query_bits_range(&v, &(3..5)), 0b11);
        assert_eq!(query_bits_range(&v, &(3..8)), 0b11101);
    }
}
