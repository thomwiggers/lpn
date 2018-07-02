use m4ri_rust::friendly::*;
use rand::distributions::{Bernoulli, Distribution};
use rand;
use std::ops::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Query {
    pub a: BinVector,
    pub s: bool,
    pub e: bool,
}

impl Query {
    pub fn count_ones(&self) -> u32 {
        self.a.count_ones()
    }
}

#[derive(Clone)]
pub struct LpnOracle {
    pub queries: Vec<Query>,
    pub secret: BinVector,
    pub k: u32,
    pub delta: f64,
    pub delta_s: f64,
    pub sparse_transform_matrix: Option<BinMatrix>,
    pub sparse_transform_vector: Option<BinVector>,
}

impl LpnOracle {
    pub fn new(k: u32, tau: f64) -> LpnOracle {
        debug_assert!(0.0 <= tau && tau < 1.0, "0 <= tau < 1");
        debug_assert!(k > 0, "k > 0");
        let secret = BinVector::random(k as usize);
        println!("Constructed Oracle with k={}, τ={:0.5}", k, tau);

        LpnOracle {
            queries: vec![],
            secret,
            k,
            delta: 1f64 - 2f64 * tau,
            delta_s: 0f64, // uniformly random
            sparse_transform_matrix: None,
            sparse_transform_vector: None,
        }
    }

    pub fn new_with_secret(secret: BinVector, tau: f64) -> LpnOracle {
        let mut lpn = Self::new(secret.len() as u32, tau);
        lpn.secret = secret;
        lpn
    }

    pub fn get_queries(&mut self, n: usize) {
        println!("Getting {} queries", n);
        self.queries.reserve(n);
        let k = self.k as usize;
        let len = if k % 8 == 0 { k / 8 } else { k / 8 + 1 };
        debug_assert!(len > 0);

        let mut rng = rand::thread_rng();
        let tau = (1.0 - self.delta) / 2.0;
        let dist = Bernoulli::new(tau);
        for _ in 0..n {
            let mut vector = BinVector::random(self.k as usize);
            debug_assert_eq!(vector.len(), self.k as usize);
            let e = dist.sample(&mut rng);

            let query = Query {
                s: &self.secret * &vector ^ e,
                a: vector,
                e,
            };
            self.queries.push(query);
        }
    }
}

pub fn query_bits_range(v: &BinVector, range: Range<usize>) -> u64 {
    // FIXME speed up
    let len = range.end - range.start;
    debug_assert!(len < 64, "Needs to fit in u64");
    let mut result = 0u64;
    for (i, ri) in range.rev().enumerate() {
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
        assert_eq!(query_bits_range(&v, 0..3), 0b100);
        assert_eq!(query_bits_range(&v, 2..4), 0b01);
        assert_eq!(query_bits_range(&v, 3..4), 0b1);
        assert_eq!(query_bits_range(&v, 3..5), 0b11);
        assert_eq!(query_bits_range(&v, 3..8), 0b11101);
    }
}
