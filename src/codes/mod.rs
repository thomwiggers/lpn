use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;
use std::collections::HashSet;
use std::fmt;
use std::mem;

pub(crate) static N: usize = 100;

fn usize_to_binvec(c: usize, size: usize) -> BinVector {
    let bytes = unsafe { mem::transmute::<usize, [u8; mem::size_of::<usize>()]>(c.to_be()) };
    let skip = (64 - size) / 8;
    let mut binvec = BinVector::from_bytes(&bytes[skip..]);
    let result = BinVector::from(binvec.split_off(((8 - skip) * 8) - size));
    debug_assert_eq!(result.len(), size);
    result
}

pub trait BinaryCode {
    /// Name of the code
    fn name(&self) -> String;

    /// Length of the code
    fn length(&self) -> usize;

    /// Dimension of the code
    fn dimension(&self) -> usize;

    /// Generator Matrix
    fn generator_matrix(&self) -> &BinMatrix;

    /// Parity check matrix
    fn parity_check_matrix(&self) -> &BinMatrix;

    /// Decode a codeword to the codeword space
    fn decode_to_code(&self, c: &BinVector) -> Result<BinVector, &str> {
        Ok(self.encode(&self.decode_to_message(c)?))
    }

    /// Decode a codeword to the message space
    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str>;

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
    fn bias(&self, delta: f64) -> f64 {
        let mut distances = Vec::with_capacity(N);
        if 2f64.powi(self.length() as i32) > 1.5 * N as f64 {
            let mut seen = HashSet::with_capacity(N);
            while seen.len() < N {
                let v = BinVector::random(self.length());
                if seen.contains(&v) {
                    continue;
                }
                let decoded = self.decode_to_code(&v);
                if let Ok(decoded) = decoded {
                    distances.push((&v + &decoded).count_ones() as i32);
                    seen.insert(v);
                } else {
                    println!("Decoding something failed");
                    return 0.0;
                }
            }
        } else {
            for i in 0..2usize.pow(self.length() as u32) {
                let v = usize_to_binvec(i, self.length());
                let decoded = self.decode_to_code(&v);
                if let Ok(decoded) = decoded {
                    distances.push((&v + &decoded).count_ones() as i32);
                } else {
                    println!("Decoding something failed");
                    return 0.0;
                }
            }
        }

        let count = distances.len();
        let sum = distances
            .into_iter()
            .fold(0f64, |acc, dist| acc + delta.powi(dist));

        sum / (count as f64)
    }
}

impl fmt::Debug for BinaryCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}] Binary Code", self.length(), self.dimension())
    }
}

mod hamming;
pub use self::hamming::*;

mod golay;
pub use self::golay::*;

mod concatenated;
pub use self::concatenated::*;

mod stgen;
pub use self::stgen::*;

mod identity;
pub use self::identity::*;

mod repetition;
pub use self::repetition::*;

mod bogosrnd;
pub use self::bogosrnd::*;
