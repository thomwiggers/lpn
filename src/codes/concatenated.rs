#![allow(clippy::mutex_atomic)]
use crate::codes::BinaryCode;
use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;
use std::cell::UnsafeCell;
use std::ptr;
use std::sync::Mutex;

/// 'Concatenated' Linear Codes
///
/// This struct allows to construct a Linear code from the direct sum
/// of smaller codes.
///
/// It will construct the generator matrix lazily and use the encode and
/// decode mechanism of the 'child' codes.
#[derive(Serialize)]
pub struct ConcatenatedCode<'a> {
    codes: Vec<&'a dyn BinaryCode>,
    #[serde(skip, default = "default_mutex")]
    init: Mutex<bool>,
    #[serde(skip, default = "default_generator")]
    generator: UnsafeCell<*mut BinMatrix>,
}

#[allow(dead_code)]
fn default_mutex() -> Mutex<bool> {
    Mutex::new(false)
}

#[allow(dead_code)]
fn default_generator() -> UnsafeCell<*mut BinMatrix> {
    UnsafeCell::new(ptr::null_mut())
}

unsafe impl<'a> Sync for ConcatenatedCode<'a> {}

impl<'codes> Clone for ConcatenatedCode<'codes> {
    fn clone(&self) -> Self {
        ConcatenatedCode {
            codes: self.codes.clone(),
            init: Mutex::new(false),
            generator: UnsafeCell::new(ptr::null_mut()),
        }
    }
}

impl<'codes> ConcatenatedCode<'codes> {
    pub fn new(codes: Vec<&'codes dyn BinaryCode>) -> ConcatenatedCode<'codes> {
        ConcatenatedCode {
            codes,
            init: Mutex::new(false),
            generator: UnsafeCell::new(ptr::null_mut()),
        }
    }
}

impl<'codes> BinaryCode for ConcatenatedCode<'codes> {
    fn name(&self) -> String {
        let names = self.codes.iter().fold(
            String::with_capacity(self.codes.iter().fold(0, |acc, c| acc + 2 + c.name().len())),
            |mut s, code| {
                s.push_str(&code.name());
                s.push_str(", ");
                s
            },
        );
        format!(
            "[{}, {}] Concatenated code codes=[{}]",
            self.length(),
            self.dimension(),
            names,
        )
    }

    fn length(&self) -> usize {
        self.codes.iter().fold(0usize, |a, c| a + c.length())
    }

    fn dimension(&self) -> usize {
        self.codes.iter().fold(0usize, |a, c| a + c.dimension())
    }

    fn generator_matrix(&self) -> &BinMatrix {
        debug_assert_ne!(
            self.codes.len(),
            0,
            "We need at least one code for this to work"
        );
        // check if we've initialized the generator
        {
            let mut initialized = self.init.lock().unwrap();
            if !*initialized {
                let mut gen = Box::new(self.codes[0].generator_matrix().clone());
                for code in self.codes.iter().skip(1) {
                    let corner = (gen.nrows(), gen.ncols());
                    gen = Box::new(gen.augmented(&BinMatrix::zero(gen.nrows(), code.length())));
                    gen = Box::new(gen.stacked(&BinMatrix::zero(code.dimension(), gen.ncols())));
                    gen.set_window(corner.0, corner.1, code.generator_matrix());
                }
                unsafe {
                    *(self.generator.get()) = Box::into_raw(gen);
                }
                *initialized = true;
            };
        }
        unsafe { (*(self.generator.get())).as_ref().unwrap() }
    }

    fn parity_check_matrix(&self) -> &BinMatrix {
        panic!("Not yet implemented");
    }

    fn encode(&self, c: &BinVector) -> BinVector {
        let mut encoded = BinVector::with_capacity(self.dimension());
        let mut slice = c.clone();
        for code in self.codes.iter() {
            let next_encode = BinVector::from(slice.split_off(code.dimension()));
            debug_assert_eq!(slice.len(), code.dimension(), "split dimension incorrect");
            encoded.extend_from_binvec(&code.encode(&slice));
            slice = next_encode;
        }
        encoded
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        let mut decoded = c.clone();
        let stor = unsafe { decoded.get_storage_mut() };
        let u64_len = stor.len() * (std::mem::size_of::<u64>() / std::mem::size_of::<usize>());
        let slice =
            unsafe { std::slice::from_raw_parts_mut(stor.as_mut_ptr() as *mut u64, u64_len) };
        self.decode_slice(slice);
        decoded.truncate(self.dimension());
        Ok(decoded)
    }

    fn decode_slice(&self, c: &mut [u64]) {
        debug_assert!(c.len() < 30, "can't deal with this long input");
        let mut bit_selection = [0u64; 30];

        let mut read_start_bit = 0;
        let mut write_start_bit = 0;
        for code in &self.codes {
            let read_range = read_start_bit..(read_start_bit + code.length());
            let blocks = read_range.len() / 64;
            for block in &mut bit_selection[..=blocks] {
                *block = 0;
            }
            for (idx, pos) in read_range.into_iter().enumerate() {
                bit_selection[idx / 64] |= ((c[pos / 64] >> (pos % 64)) & 1) << (idx % 64);
            }

            code.decode_slice(&mut bit_selection[..=blocks]);

            let write_range = write_start_bit..(write_start_bit + code.dimension());
            for (idx, pos) in write_range.into_iter().enumerate() {
                let mask = !(1 << (pos % 64));
                c[pos / 64] = (c[pos / 64] & mask)
                    | (!mask & ((bit_selection[idx / 64] >> (idx % 64)) << (pos % 64)))
            }
            read_start_bit += code.length();
            write_start_bit += code.dimension();
        }
        // mask last block
        c[write_start_bit / 64] &= (1 << (write_start_bit % 64)) - 1;
    }

    fn bias(&self, delta: f64) -> f64 {
        self.codes
            .iter()
            .fold(1f64, |acc, code| acc * code.bias(delta))
    }
}

#[cfg(feature = "hamming")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::codes::hamming::*;
    use m4ri_rust::friendly::BinVector;

    fn get_code() -> ConcatenatedCode<'static> {
        let codes: Vec<&dyn BinaryCode> = vec![&HammingCode7_4, &HammingCode3_1];
        ConcatenatedCode::new(codes)
    }

    #[test]
    fn test_concatenated_code() {
        let code = get_code();

        assert_eq!(code.length(), 7 + 3, "Length wrong");
        assert_eq!(code.dimension(), 4 + 1, "Dimension wrong");

        let mut input = BinVector::from_bytes(&[0b1010_1000]);
        input.truncate(5);
        assert_eq!(
            input,
            BinVector::from_bools(&[true, false, true, false, true])
        );

        let mut encoded = BinVector::from_bytes(&[0b1010_1011, 0b1100_0000]);
        encoded.truncate(10);
        assert_eq!(
            *encoded,
            *code.encode(&input),
            "encode(input) doesn't match encoded"
        );

        // idempotent
        assert_eq!(
            input,
            code.decode_to_message(&code.encode(&input)).unwrap(),
            "not idempotent"
        );
    }

    #[test]
    fn test_random_samples() {
        let code = ConcatenatedCode::new(vec![&HammingCode15_11, &HammingCode7_4, &HammingCode3_1]);

        for _ in 0..100 {
            let v = BinVector::random(code.length());
            let x = code.decode_to_message(&v).unwrap();
            let cw = code.decode_to_code(&v).unwrap();
            assert_eq!(&code.encode(&x), &cw);
            assert!((v + cw).count_ones() < 5);
        }
    }
}
