use std::collections::HashSet;
use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;
use std::mem;

static N: usize = 100000;

fn usize_to_binvec(c: usize, size: usize) -> BinVector {
    let bytes = unsafe { mem::transmute::<usize, [u8; mem::size_of::<usize>()]>(c.to_be()) };
    let skip = (64 - size) / 8;
    let mut binvec = BinVector::from_bytes(&bytes[skip..]);
    let result = BinVector::from(binvec.split_off(((8 - skip) * 8) - size));
    debug_assert_eq!(result.len(), size);
    result
}

pub trait BinaryCode<'a> {
    /// Length of the code
    fn length(&self) -> usize;

    /// Dimension of the code
    fn dimension(&self) -> usize;

    /// Generator Matrix
    fn generator_matrix(&self) -> &'a BinMatrix;

    /// Parity check matrix
    fn parity_check_matrix(&self) -> &'a BinMatrix;

    /// Decode a codeword to the codeword space
    fn decode_to_code(&self, c: &BinVector) -> BinVector {
        self.encode(&self.decode_to_message(c))
    }

    /// Decode a codeword to the message space
    fn decode_to_message(&self, c: &BinVector) -> BinVector;

    /// Encode a codeword
    fn encode(&self, c: &BinVector) -> BinVector {
        debug_assert_eq!(
            c.len(),
            self.dimension(),
            "Vector to encode should be of length {}",
            self.dimension()
        );
        let result = c * self.generator_matrix();
        debug_assert_eq!(
            result.len(),
            self.length(),
            "wtf, product should be of length"
        );
        result
    }

    /// Get or compute the bc of a code
    fn bias(&self, delta: f64) -> f64{
        let mut distances = Vec::with_capacity(N);
        if 2f64.powi(self.length() as i32) > 1.5 * N as f64 {
            let mut seen = HashSet::new();
            while seen.len() < N {
                let v = BinVector::random(self.length());
                if seen.contains(&v) {
                    continue;
                }
                distances.push((&v + &self.decode_to_code(&v)).count_ones() as i32);
                seen.insert(v);
            }
        } else {
            for i in 0..2usize.pow(self.length() as u32) {
                let v = usize_to_binvec(i, self.length());
                distances.push((&v + &self.decode_to_code(&v)).count_ones() as i32);
            }
        }

        let count = distances.len();
        let sum = distances.into_iter().fold(0f64, |acc, dist| acc + delta.powi(dist));

        sum / (count as f64)
    }
}

mod hamming;
pub use self::hamming::*;

mod concatenated;
pub use self::concatenated::*;

mod stgen;
pub use self::stgen::*;
