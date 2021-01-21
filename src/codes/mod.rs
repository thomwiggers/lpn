//! This module defines Linear codes for the covering-codes reduction.
use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;
use std::collections::HashSet;
use std::fmt;
use std::mem;

use crate::oracle::Sample;

/// Sample size to estimate the covering radius
pub(crate) static N: usize = 10000;

fn usize_to_binvec(c: usize, size: usize) -> BinVector {
    let bytes = unsafe { mem::transmute::<usize, [u8; mem::size_of::<usize>()]>(c.to_be()) };
    let skip = (64 - size) / 8;
    let mut binvec = BinVector::from_bytes(&bytes[skip..]);
    let result = BinVector::from(binvec.split_off(((8 - skip) * 8) - size));
    debug_assert_eq!(result.len(), size);
    result
}

/// Generic binary linear code API
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

    fn decode_sample(&self, c: &mut Sample) {
        use crate::oracle::{NOISE_BIT_BLOCK, NOISE_BIT_MASK};
        let slice = c.get_sample_mut();
        if NOISE_BIT_BLOCK == self.length() / 64 {
            // we're going to be touching the last block
            let noise_bit = slice[NOISE_BIT_BLOCK] & NOISE_BIT_MASK;
            slice[NOISE_BIT_BLOCK] &= !NOISE_BIT_MASK;
            self.decode_slice(slice);
            // truncate
            slice[NOISE_BIT_BLOCK] &= (1 << self.dimension()) - 1;
            // restore noise bit
            slice[NOISE_BIT_BLOCK] |= noise_bit;
        } else {
            self.decode_slice(&mut slice[..=self.length() / 64]);
            c.truncate(self.dimension(), false)
        }
    }

    fn decode_slice(&self, c: &mut [u64]) {
        let mut v = BinVector::with_capacity(self.length());
        let stor = unsafe { v.get_storage_mut() };
        stor.extend(c.iter().copied().map(|b| b as usize));
        let v = self.decode_to_message(&v).unwrap();
        c.iter_mut()
            .zip(v.get_storage().iter().copied())
            .for_each(|(b, d)| {
                *b = d as u64;
            });
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

impl fmt::Debug for dyn BinaryCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}] Binary Code", self.length(), self.dimension())
    }
}

impl serde::Serialize for &dyn BinaryCode {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        ser.serialize_str(&self.name())
    }
}

#[cfg(feature = "hamming")]
mod hamming;
#[cfg(feature = "hamming")]
pub use self::hamming::*;

#[cfg(feature = "golay")]
mod golay;
#[cfg(feature = "golay")]
pub use self::golay::*;

mod concatenated;
pub use self::concatenated::*;

#[cfg(feature = "stgen")]
mod stgen;
#[cfg(feature = "stgen")]
pub use self::stgen::*;

mod identity;
pub use self::identity::*;

mod repetition;
pub use self::repetition::*;

#[cfg(feature = "bogosrnd")]
mod bogosrnd;
#[cfg(feature = "bogosrnd")]
pub use self::bogosrnd::*;

#[cfg(feature = "mds")]
mod mds;
#[cfg(feature = "mds")]
pub use self::mds::*;

#[cfg(feature = "custom")]
mod custom;
#[cfg(feature = "custom")]
pub use self::custom::*;

#[cfg(feature = "wagner")]
mod wagner;
#[cfg(feature = "wagner")]
pub use self::wagner::*;

#[cfg(feature = "guavacodes")]
mod guava;
#[cfg(feature = "guavacodes")]
pub use self::guava::*;