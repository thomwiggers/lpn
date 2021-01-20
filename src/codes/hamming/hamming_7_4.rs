use std::sync::Once;
use std::boxed::Box;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;
use crate::oracle::{Sample, SAMPLE_LEN};

/// ``[7, 4]`` Hamming code
/// 
/// Decodes using direct lookup tables
#[derive(Debug,Serialize)]
pub struct HammingCode7_4;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[97],
                &[82],
                &[52],
                &[120],
                
            ], 7));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                    &[85],
                    &[102],
                    &[120],
                    
            ], 7));
            PARITY_MATRIX = Box::into_raw(matrix);
        }
    });
}

// only supports 1-block syndromes
static SYNDROME: [usize; 128] = [
     0, // [0]
     0, // [0]
     0, // [0]
     7, // [7]
     0, // [0]
     7, // [7]
     7, // [7]
     7, // [7]
     0, // [0]
     9, // [9]
     10, // [10]
     11, // [11]
     12, // [12]
     13, // [13]
     14, // [14]
     7, // [7]
     0, // [0]
     9, // [9]
     2, // [2]
     3, // [3]
     4, // [4]
     5, // [5]
     14, // [14]
     7, // [7]
     9, // [9]
     9, // [9]
     14, // [14]
     9, // [9]
     14, // [14]
     9, // [9]
     14, // [14]
     14, // [14]
     0, // [0]
     1, // [1]
     10, // [10]
     3, // [3]
     4, // [4]
     13, // [13]
     6, // [6]
     7, // [7]
     10, // [10]
     13, // [13]
     10, // [10]
     10, // [10]
     13, // [13]
     13, // [13]
     10, // [10]
     13, // [13]
     4, // [4]
     3, // [3]
     3, // [3]
     3, // [3]
     4, // [4]
     4, // [4]
     4, // [4]
     3, // [3]
     8, // [8]
     9, // [9]
     10, // [10]
     3, // [3]
     4, // [4]
     13, // [13]
     14, // [14]
     15, // [15]
     0, // [0]
     1, // [1]
     2, // [2]
     11, // [11]
     12, // [12]
     5, // [5]
     6, // [6]
     7, // [7]
     12, // [12]
     11, // [11]
     11, // [11]
     11, // [11]
     12, // [12]
     12, // [12]
     12, // [12]
     11, // [11]
     2, // [2]
     5, // [5]
     2, // [2]
     2, // [2]
     5, // [5]
     5, // [5]
     2, // [2]
     5, // [5]
     8, // [8]
     9, // [9]
     2, // [2]
     11, // [11]
     12, // [12]
     5, // [5]
     14, // [14]
     15, // [15]
     1, // [1]
     1, // [1]
     6, // [6]
     1, // [1]
     6, // [6]
     1, // [1]
     6, // [6]
     6, // [6]
     8, // [8]
     1, // [1]
     10, // [10]
     11, // [11]
     12, // [12]
     13, // [13]
     6, // [6]
     15, // [15]
     8, // [8]
     1, // [1]
     2, // [2]
     3, // [3]
     4, // [4]
     5, // [5]
     6, // [6]
     15, // [15]
     8, // [8]
     8, // [8]
     8, // [8]
     15, // [15]
     8, // [8]
     15, // [15]
     15, // [15]
     15, // [15]
];

static ENCODE: [usize; 16] = [
       0, // [0]
       97, // [97]
       82, // [82]
       51, // [51]
       52, // [52]
       85, // [85]
       102, // [102]
       7, // [7]
       120, // [120]
       25, // [25]
       42, // [42]
       75, // [75]
       76, // [76]
       45, // [45]
       30, // [30]
       127, // [127]
];


impl BinaryCode for HammingCode7_4 {
    fn name(&self) -> String {
        "[7, 4] Hamming code".to_owned()
    }

    fn length(&self) -> usize {
        7
    }

    fn dimension(&self) -> usize {
        4
    }

    fn generator_matrix(&self) -> &BinMatrix {
        init();
        unsafe {
            GENERATOR_MATRIX.as_ref().unwrap()
        }
    }

    fn parity_check_matrix(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX.as_ref().unwrap()
        }
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        debug_assert_eq!(c.len(), self.length());
        let mut v = BinVector::with_capacity(self.dimension());
        let stor = unsafe { v.get_storage_mut() };
        stor.push(SYNDROME[c.as_u32() as usize]);
        unsafe { v.set_len(self.dimension()); }
        Ok(v)
    }

    /// Encode using lookup table
    fn encode(&self, c: &BinVector) -> BinVector {
        debug_assert_eq!(c.len(), self.dimension());
        let mut v = BinVector::with_capacity( self.length() );
        let stor = unsafe { v.get_storage_mut() };
        stor.push(ENCODE[c.as_u32() as usize]);
        unsafe { v.set_len(self.length()); }
        v
    }

    /// Decode a Sample
    #[inline]
    fn decode_sample(&self, sample: &mut Sample) {
        assert!(self.length() < 64);
        const MASK: u64 = (1 << 7) - 1;
        let c = sample.get_sample_mut();
        if SAMPLE_LEN == 1 { // need to ignore product
            c[0] = (c[0] & !MASK) | (SYNDROME[(c[0] & MASK) as usize] as u64);
        } else {
            self.decode_slice(c)
        }
    }

    #[inline]
    fn decode_slice(&self, c: &mut [u64]) {
        debug_assert_eq!(c[0] & !((1 << self.length()) - 1), 0, "this message is too long");

        c[0] = SYNDROME[c[0] as usize] as u64
    }

    // for hamming codes
    fn bias(&self, delta: f64) -> f64 {
        (1f64 + f64::from(7) * delta) / f64::from(7 + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use m4ri_rust::friendly::BinVector;

    #[test]
    fn size() {
        let code = HammingCode7_4.generator_matrix();
        assert_eq!(code.ncols(), 7);
        assert_eq!(code.nrows(), 4);
    }

    #[test]
    fn decode() {
        let code = HammingCode7_4;

        let codeword = code.decode_to_message(&BinVector::from_elem(7, true)).unwrap();
        assert_eq!(codeword, BinVector::from_elem(4, true));

        let mut vec = BinVector::from_elem(7, true);
        vec.set(0, false);
        let codeword = code.decode_to_message(&vec).unwrap();
        assert_eq!(codeword, BinVector::from_elem(4, true));

        let vec = code.decode_to_code(&BinVector::from_elem(7, false)).unwrap();
        assert_eq!(vec, BinVector::from_elem(7, false));
    }

    #[test]
    fn test_decode_sample() {
        let code = HammingCode7_4;
        for _ in 0..1000 {
            // setup
            let vec = BinVector::random(code.length());
            let mut sample_a = Sample::from_binvector(&vec, false);
            let mut sample_b = Sample::from_binvector(&vec, true);
            
            let decoded_vec = code.decode_to_message(&vec).unwrap();
            println!("decoded_vec: {:?}", decoded_vec);

            // test vectors
            let decoded_vec_sample_a = Sample::from_binvector(&decoded_vec, false);
            let decoded_vec_sample_b = Sample::from_binvector(&decoded_vec, true);

            code.decode_sample(&mut sample_a);
            code.decode_sample(&mut sample_b);
            assert_eq!(sample_a.get_product(), false);
            assert_eq!(sample_b.get_product(), true);
            assert_eq!(sample_a, decoded_vec_sample_a);
            assert_eq!(sample_b, decoded_vec_sample_b);
        }
    }

}
