use binomial_iter::BinomialIter;
use codes::{BinaryCode, N};
use m4ri_rust::friendly::*;
use std::cmp;

use std::collections::HashSet;

pub struct RepetitionCode {
    k: usize,
    generator: BinMatrix,
}

impl cmp::PartialEq for RepetitionCode {
    fn eq(&self, other: &RepetitionCode) -> bool {
        self.k == other.k
    }
}

impl cmp::Eq for RepetitionCode {}

impl RepetitionCode {
    pub fn new(k: usize) -> RepetitionCode {
        RepetitionCode {
            k,
            generator: BinVector::from_elem(k, true).as_matrix(),
        }
    }
}

#[inline]
fn choose(n: usize, k: usize) -> f64 {
    BinomialIter::new(n as u32, k as u32).binom() as f64
}

impl BinaryCode for RepetitionCode {
    fn name(&self) -> String {
        format!("[{k}, 1] trivial linear code", k = self.k)
    }

    fn length(&self) -> usize {
        self.k
    }

    fn dimension(&self) -> usize {
        1
    }

    fn generator_matrix(&self) -> &BinMatrix {
        &self.generator
    }

    fn parity_check_matrix(&self) -> &BinMatrix {
        panic!("not yet implemented");
    }

    fn decode_to_code(&self, c: &BinVector) -> Result<BinVector, &str> {
        let bit = c.count_ones() > ((self.k / 2) as u32);
        Ok(BinVector::from_elem(self.k, bit))
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        let bit = c.count_ones() > ((self.k / 2) as u32);
        Ok(BinVector::from_elem(1, bit))
    }

    /// Directly compute the bias of repetition codes using
    /// the formulae of Bogos / Tramer / Vaudenay (2015)
    ///
    /// https://eprint.iacr.org/2015/049
    fn bias(&self, delta: f64) -> f64 {
        if self.k > 33 {
            return self.measure_bias(delta);
        }
        if self.k % 2 == 1 {
            // perfect code
            (0..((self.k - 1) / 2 + 1))
                .into_iter()
                .fold(0f64, |acc, w| {
                    acc + choose(self.k, w) * delta.powi(w as i32)
                }) / 2f64.powi((self.k - 1) as i32)
        } else {
            (0..(self.k / 2)).into_iter().fold(0f64, |acc, w| {
                acc + choose(self.k, w) * delta.powi(w as i32)
            }) / 2f64.powi((self.k - 1) as i32)
                + 1f64 / 2f64.powi(self.k as i32)
                    * choose(self.k, self.k / 2)
                    * delta.powi((self.k / 2) as i32)
        }
    }
}

impl RepetitionCode {
    fn measure_bias(&self, delta: f64) -> f64 {
        let mut distances = Vec::with_capacity(N);
        let mut seen = HashSet::with_capacity(N);
        while seen.len() < cmp::min(N, 2usize.pow(self.length() as u32)) {
            let v = BinVector::random(self.length());
            if seen.contains(&v) {
                continue;
            }
            let decoded = self.decode_to_code(&v).unwrap();
            distances.push((&v + &decoded).count_ones() as i32);
            seen.insert(v);
        }

        let count = distances.len();
        let sum = distances
            .into_iter()
            .fold(0f64, |acc, dist| acc + delta.powi(dist));

        sum / (count as f64)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bias_even() {
        let code = RepetitionCode::new(2);
        let delta = 1.0 / 8.0;
        assert_eq!(code.bias(delta), 0.5 * delta + 0.5);

        let code = RepetitionCode::new(4);
        assert_eq!(
            code.bias(delta),
            3.0 / 8.0 * delta.powi(2) + 0.5 * delta + 1.0 / 8.0
        );

        let code = RepetitionCode::new(8);
        assert_eq!(
            code.bias(delta),
            35.0 / 128.0 * delta.powi(4)
                + 7.0 / 16.0 * delta.powi(3)
                + 7.0 / 32.0 * delta.powi(2)
                + 1.0 / 16.0 * delta
                + 1.0 / 128.0
        );
    }

    #[test]
    fn test_bias_odd() {
        let delta = 1.0 / 8.0;
        let code = RepetitionCode::new(3);
        assert_eq!(code.bias(delta), 0.75 * delta + 0.25, "odd bc incorrect");

        let code = RepetitionCode::new(9);
        assert_eq!(
            code.bias(delta),
            63.0 / 128.0 * delta.powi(4)
                + 21.0 / 64.0 * delta.powi(3)
                + 9.0 / 64.0 * delta.powi(2)
                + 9.0 / 256.0 * delta
                + 1.0 / 256.0
        );
    }
}
