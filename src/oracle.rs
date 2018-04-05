use bit_vec::BitVec;
use rand;
use rand::Rng;
use std::ops::Range;

pub struct Query {
    pub a: BitVec,
    pub s: bool,
}

impl Query {
    pub fn count_ones(&self) -> u32 {
        self.a
            .storage()
            .iter()
            .fold(0, |acc, block| acc + block.count_ones())
    }
}

pub struct LpnOracle {
    pub queries: Vec<Query>,
    pub secret: BitVec,
    pub k: u32,
    pub tau: f32,
}

impl LpnOracle {
    pub fn new(k: u32, tau: f32) -> LpnOracle {
        debug_assert!(0.0 <= tau && tau < 1.0, "0 <= tau < 1");
        debug_assert!(k > 0, "k > 0");
        let mut rng = rand::thread_rng();
        let len = if k % 8 == 0 { k / 8 } else { k / 8 + 1 };

        let mut secret_bytes: Vec<u8> = Vec::with_capacity(len as usize);
        for _ in 0..len {
            secret_bytes.push(rng.gen());
        }
        let mut secret = BitVec::from_bytes(&secret_bytes);
        secret.truncate(k as usize);
        debug_assert_eq!(secret.len(), k as usize);

        LpnOracle {
            queries: vec![],
            secret,
            k,
            tau,
        }
    }

    pub fn get_queries(&mut self, n: usize) {
        println!("Getting {} queries", n);
        self.queries.reserve(n);
        let k = self.k as usize;
        let len = if k % 8 == 0 { k / 8 } else { k / 8 + 1 };
        debug_assert!(len > 0);

        let mut rng = rand::thread_rng();
        for _ in 0..n {
            let mut blocks: Vec<u8> = Vec::with_capacity(len);
            for _ in 0..len {
                blocks.push(rng.gen());
            }
            let mut vector = BitVec::from_bytes(&blocks);
            vector.truncate(self.k as usize);
            debug_assert_eq!(vector.len(), self.k as usize);

            let query = Query {
                s: vector_product(&self.secret, &vector),
                a: vector,
            };
            self.queries.push(query);
        }
    }
}

pub fn vector_weight(vector: &BitVec) -> u32 {
    vector.storage().iter().fold(0u32, |acc, block| acc + block.count_ones())
}

/// a * b
pub fn vector_product(a: &BitVec, b: &BitVec) -> bool {
    let mut sum = false;
    let mut z = a.clone();
    z.intersect(b);
    if z.storage()
        .iter()
        .fold(0u32, |acc, block| acc + block.count_ones()) % 2 == 1
    {
        // odd
        sum ^= true;
    }
    sum
}

/// a ^= b
pub fn vector_sum(a: &mut BitVec, b: &BitVec) {
    unsafe {
        let storage = a.storage_mut();
        for (mut a, b) in storage.iter_mut().zip(b.storage().iter()) {
            *a ^= b;
        }
    }
}

pub fn random_vector() {
    rand::thread_rng();
}
pub fn query_bits_range(v: &BitVec, range: Range<usize>) -> u64 {
    let len = range.end - range.start;
    debug_assert!(len < 64, "Needs to fit in u64");
    let mut result = 0u64;
    for (i, ri) in range.rev().enumerate() {
        result ^= (v.get(ri).expect("Index out of bounds") as u64) << i;
    }
    debug_assert_eq!(result >> len, 0);
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bitrange() {
        let v = BitVec::from_bytes(&[0b10011101u8]);
        assert_eq!(query_bits_range(&v, 0..3), 0b100);
        assert_eq!(query_bits_range(&v, 2..4), 0b01);
        assert_eq!(query_bits_range(&v, 3..4), 0b1);
        assert_eq!(query_bits_range(&v, 3..5), 0b11);
        assert_eq!(query_bits_range(&v, 3..8), 0b11101);
    }
}
