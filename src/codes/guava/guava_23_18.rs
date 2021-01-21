use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[23, 18]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode23_18;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 2359297 ],
                &[ 1310722 ],
                &[ 7602180 ],
                &[ 786440 ],
                &[ 7077904 ],
                &[ 6029344 ],
                &[ 3932224 ],
                &[ 8126592 ],
                &[ 1835264 ],
                &[ 2884096 ],
                &[ 4981760 ],
                &[ 3409920 ],
                &[ 5509120 ],
                &[ 6561792 ],
                &[ 3686400 ],
                &[ 5799936 ],
                &[ 6881280 ],
                &[ 7471104 ],
                
            ], 23));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 6348385 ],
                &[ 5336402 ],
                &[ 3951412 ],
                &[ 7747704 ],
                &[ 8388480 ],
                
            ], 23));
            let matrix_t = Box::new(matrix.transposed());
            PARITY_MATRIX = Box::into_raw(matrix);
            PARITY_MATRIX_T = Box::into_raw(matrix_t);

            let mut map = Box::new(FnvHashMap::with_capacity_and_hasher(32, Default::default()));
            map.insert(0, &[0]);     // 0 => [0]
            map.insert(1, &[1]);     // 1 => [1]
            map.insert(2, &[2]);     // 2 => [2]
            map.insert(4, &[4]);     // 4 => [4]
            map.insert(8, &[8]);     // 8 => [8]
            map.insert(14, &[16]);     // 14 => [16]
            map.insert(13, &[32]);     // 13 => [32]
            map.insert(11, &[64]);     // 11 => [64]
            map.insert(16, &[128]);     // 16 => [128]
            map.insert(22, &[256]);     // 22 => [256]
            map.insert(21, &[512]);     // 21 => [512]
            map.insert(19, &[1024]);     // 19 => [1024]
            map.insert(31, &[2048]);     // 31 => [2048]
            map.insert(25, &[4096]);     // 25 => [4096]
            map.insert(26, &[8192]);     // 26 => [8192]
            map.insert(23, &[16384]);     // 23 => [16384]
            map.insert(17, &[32768]);     // 17 => [32768]
            map.insert(18, &[65536]);     // 18 => [65536]
            map.insert(24, &[131072]);     // 24 => [131072]
            map.insert(28, &[262144]);     // 28 => [262144]
            map.insert(20, &[524288]);     // 20 => [524288]
            map.insert(30, &[1048576]);     // 30 => [1048576]
            map.insert(29, &[2097152]);     // 29 => [2097152]
            map.insert(27, &[4194304]);     // 27 => [4194304]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(15, &[17]);     // 15 => [17]
            map.insert(12, &[33]);     // 12 => [33]
            map.insert(10, &[65]);     // 10 => [65]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(7, &[16512]);     // 7 => [16512]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode23_18 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode23_18 {
    fn name(&self) -> String {
        "[23, 18] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        23
    }

    fn dimension(&self) -> usize {
        18
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

    fn decode_to_code(&self, c: &BinVector) -> Result<BinVector, &str> {
        init();
        let map = unsafe {
            SYNDROME_MAP.as_ref().unwrap()
        };
        debug_assert_eq!(c.len(), self.length(), "the length doesn't match the expected length (length of the code)");
        let he = c * self.parity_check_matrix_transposed();
        let mut error = BinVector::with_capacity(23);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 23 / 64 + if 23 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(23) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(18);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[23 / 64] & !((1 << 23) - 1), 0, "this message has excess bits");

        let map = unsafe {
            SYNDROME_MAP.as_ref().unwrap()
        };
        let he = &BinMatrix::from_slices(&[&c[..]], self.length()) * self.parity_check_matrix_transposed();
        let error = map[unsafe { &he.get_word_unchecked(0, 0) }];
        c.iter_mut().zip(error.iter().copied()).for_each(|(sample, error)| *sample ^= error as u64);
    }

    
}

#[cfg(test)]
mod tests {
    use super::*;
    use m4ri_rust::friendly::BinVector;
    use crate::oracle::Sample;

    #[test]
    fn size() {
        let code = GuavaCode23_18.generator_matrix();
        assert_eq!(code.ncols(), 23);
        assert_eq!(code.nrows(), 18);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode23_18;
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

    #[test]
    fn random_decode_tests() {

        
        {
            let code = GuavaCode23_18;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, false, false, false, true, false, true, true, true, true, true, false, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, false, false, false, true, false, true, true, true, true, true, false, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_18;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, false, true, true, true, true, true, false, false, true, true, false, false, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, false, false, false, true, true, true, true, false, false, false, true, false, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_18;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, true, true, false, true, false, true, true, false, false, true, true, false, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, true, true, false, true, true, true, true, false, false, true, true, false, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_18;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, true, false, false, false, true, false, true, false, true, false, true, true, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, true, false, false, false, true, false, true, false, true, false, true, true, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_18;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, false, true, false, true, true, true, true, true, false, true, false, false, false, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, false, true, false, true, true, true, true, true, false, true, false, false, false, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_18;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, false, false, true, false, false, false, false, false, true, false, false, false, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, false, false, true, false, true, false, false, false, true, false, false, false, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_18;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, true, true, false, true, true, false, true, false, true, true, true, false, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, true, true, true, true, true, false, true, false, true, true, true, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_18;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, true, true, true, true, true, true, true, false, false, false, true, false, true, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, true, true, true, true, true, true, false, false, false, true, false, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_18;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, false, false, true, true, true, false, true, true, true, false, true, false, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, false, false, false, true, true, true, false, true, true, true, false, true, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_18;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, true, false, false, false, false, false, true, false, true, true, false, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, true, false, true, false, false, false, true, false, true, true, false, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_18;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, true, false, false, false, true, false, true, true, true, false, false, false, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, true, false, false, false, true, false, true, true, true, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_18;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, true, true, false, false, false, true, true, false, false, false, true, false, true, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, true, false, false, false, true, true, false, false, false, true, false, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_18;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, false, true, false, false, true, false, true, true, false, false, false, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, false, false, true, false, false, true, false, true, true, false, false, false, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_18;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, true, true, true, true, false, false, false, false, true, false, true, true, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, true, true, true, true, true, false, false, false, false, true, false, true, true, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_18;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, true, false, true, false, false, false, false, true, true, true, true, true, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, true, true, false, false, false, false, false, false, true, true, true, true, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_18;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, true, true, true, false, true, true, false, false, true, false, false, true, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, true, false, false, true, true, false, false, true, true, false, true, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_18;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, true, true, true, true, true, true, false, true, false, false, false, false, true, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, true, false, true, true, true, false, true, false, false, false, false, true, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_18;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, false, false, false, false, false, true, true, false, true, false, false, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, false, false, false, false, false, true, true, false, true, false, false, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_18;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, false, true, false, true, false, false, false, true, false, false, false, false, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, false, true, false, true, false, false, false, true, false, false, false, false, true, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_18;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, false, true, true, true, true, true, false, true, false, true, true, true, true, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, false, true, true, true, true, true, false, true, false, true, true, false, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
