use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[24, 20]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode24_20;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 1048577 ],
                &[ 1048578 ],
                &[ 1048580 ],
                &[ 1048584 ],
                &[ 1048592 ],
                &[ 1048608 ],
                &[ 1048640 ],
                &[ 1048704 ],
                &[ 1048832 ],
                &[ 1049088 ],
                &[ 1049600 ],
                &[ 1050624 ],
                &[ 1052672 ],
                &[ 1056768 ],
                &[ 1064960 ],
                &[ 1081344 ],
                &[ 1114112 ],
                &[ 1179648 ],
                &[ 1310720 ],
                &[ 1572864 ],
                
            ], 24));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 2097151 ],
                &[ 2097152 ],
                &[ 4194304 ],
                &[ 8388608 ],
                
            ], 24));
            let matrix_t = Box::new(matrix.transposed());
            PARITY_MATRIX = Box::into_raw(matrix);
            PARITY_MATRIX_T = Box::into_raw(matrix_t);

            let mut map = Box::new(FnvHashMap::with_capacity_and_hasher(16, Default::default()));
            map.insert(0, &[0]);     // 0 => [0]
            map.insert(1, &[1]);     // 1 => [1]
            map.insert(2, &[2097152]);     // 2 => [2097152]
            map.insert(4, &[4194304]);     // 4 => [4194304]
            map.insert(8, &[8388608]);     // 8 => [8388608]
            map.insert(3, &[2097153]);     // 3 => [2097153]
            map.insert(5, &[4194305]);     // 5 => [4194305]
            map.insert(9, &[8388609]);     // 9 => [8388609]
            map.insert(6, &[6291456]);     // 6 => [6291456]
            map.insert(10, &[10485760]);     // 10 => [10485760]
            map.insert(12, &[12582912]);     // 12 => [12582912]
            map.insert(7, &[6291457]);     // 7 => [6291457]
            map.insert(11, &[10485761]);     // 11 => [10485761]
            map.insert(13, &[12582913]);     // 13 => [12582913]
            map.insert(14, &[14680064]);     // 14 => [14680064]
            map.insert(15, &[14680065]);     // 15 => [14680065]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode24_20 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode24_20 {
    fn name(&self) -> String {
        "[24, 20] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        24
    }

    fn dimension(&self) -> usize {
        20
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
        let mut error = BinVector::with_capacity(24);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 24 / 64 + if 24 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(24) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(20);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[24 / 64] & !((1 << 24) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode24_20.generator_matrix();
        assert_eq!(code.ncols(), 24);
        assert_eq!(code.nrows(), 20);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode24_20;
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
            let code = GuavaCode24_20;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, false, false, false, false, true, false, false, true, false, true, true, false, false, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, false, false, false, false, true, false, false, true, false, true, true, false, false, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_20;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, true, false, false, true, false, false, false, true, true, true, false, true, false, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, false, true, false, false, true, false, false, false, true, true, true, false, true, false, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_20;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, false, false, false, false, true, true, true, true, true, false, true, true, true, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, false, false, false, false, true, true, true, true, true, false, true, true, true, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_20;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, true, true, true, false, true, true, true, false, false, true, false, true, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, true, true, true, false, true, true, true, false, false, true, false, true, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_20;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, false, true, false, true, true, true, true, false, true, false, false, true, false, true, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, true, false, true, true, true, true, false, true, false, false, true, false, true, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_20;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, true, false, true, true, false, true, false, false, true, false, false, true, true, false, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, false, true, true, false, true, false, false, true, false, false, true, true, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_20;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, true, true, true, false, true, false, false, false, true, true, false, true, true, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, true, true, true, false, true, false, false, false, true, true, false, true, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_20;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, true, true, false, true, true, false, true, false, false, false, false, false, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, true, true, true, false, true, true, false, true, false, false, false, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_20;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, true, false, false, true, true, false, true, true, true, true, false, true, false, true, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, true, false, false, true, true, false, true, true, true, true, false, true, false, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_20;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, true, false, true, true, false, false, true, false, true, true, true, false, true, false, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, true, false, true, true, false, false, true, false, true, true, true, false, true, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_20;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, false, false, true, false, true, true, false, true, true, true, false, true, false, false, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, false, false, true, false, true, true, false, true, true, true, false, true, false, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_20;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, false, false, true, false, false, false, true, true, false, true, true, true, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, false, false, false, true, false, false, false, true, true, false, true, true, true, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_20;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, false, true, true, true, true, false, false, true, false, false, true, true, true, false, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, true, true, true, true, false, false, true, false, false, true, true, true, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_20;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, false, false, false, false, false, false, false, true, false, true, true, false, true, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, false, false, false, false, false, false, true, false, true, true, false, true, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_20;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, true, true, false, false, false, false, true, false, false, true, true, false, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, true, true, false, false, false, false, true, false, false, true, true, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_20;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, false, true, true, true, false, false, true, true, false, false, false, false, true, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, false, true, true, true, false, false, true, true, false, false, false, false, true, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_20;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, true, false, true, true, false, false, false, true, false, false, true, true, true, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, true, false, true, true, false, false, false, true, false, false, true, true, true, false, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_20;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, true, false, false, false, false, false, false, true, true, false, false, true, true, true, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, true, false, false, false, false, false, false, true, true, false, false, true, true, true, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_20;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, true, true, true, true, false, false, true, true, false, true, false, true, true, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, true, true, true, true, false, false, true, true, false, true, false, true, true, true, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_20;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, true, false, false, true, false, false, false, false, false, true, false, true, false, false, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, false, false, true, false, false, false, false, false, true, false, true, false, false, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
