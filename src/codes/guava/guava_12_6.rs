use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[12, 6]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode12_6;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 2369 ],
                &[ 2626 ],
                &[ 3140 ],
                &[ 2440 ],
                &[ 2704 ],
                &[ 3232 ],
                
            ], 12));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 3761 ],
                &[ 530 ],
                &[ 1060 ],
                &[ 184 ],
                &[ 2240 ],
                &[ 3840 ],
                
            ], 12));
            let matrix_t = Box::new(matrix.transposed());
            PARITY_MATRIX = Box::into_raw(matrix);
            PARITY_MATRIX_T = Box::into_raw(matrix_t);

            let mut map = Box::new(FnvHashMap::with_capacity_and_hasher(64, Default::default()));
            map.insert(0, &[0]);     // 0 => [0]
            map.insert(1, &[1]);     // 1 => [1]
            map.insert(2, &[2]);     // 2 => [2]
            map.insert(4, &[4]);     // 4 => [4]
            map.insert(8, &[8]);     // 8 => [8]
            map.insert(11, &[16]);     // 11 => [16]
            map.insert(13, &[32]);     // 13 => [32]
            map.insert(16, &[64]);     // 16 => [64]
            map.insert(25, &[128]);     // 25 => [128]
            map.insert(32, &[256]);     // 32 => [256]
            map.insert(35, &[512]);     // 35 => [512]
            map.insert(37, &[1024]);     // 37 => [1024]
            map.insert(49, &[2048]);     // 49 => [2048]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(10, &[17]);     // 10 => [17]
            map.insert(12, &[33]);     // 12 => [33]
            map.insert(17, &[65]);     // 17 => [65]
            map.insert(24, &[129]);     // 24 => [129]
            map.insert(33, &[257]);     // 33 => [257]
            map.insert(34, &[513]);     // 34 => [513]
            map.insert(36, &[1025]);     // 36 => [1025]
            map.insert(48, &[2049]);     // 48 => [2049]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(15, &[34]);     // 15 => [34]
            map.insert(18, &[66]);     // 18 => [66]
            map.insert(27, &[130]);     // 27 => [130]
            map.insert(39, &[1026]);     // 39 => [1026]
            map.insert(51, &[2050]);     // 51 => [2050]
            map.insert(20, &[68]);     // 20 => [68]
            map.insert(29, &[132]);     // 29 => [132]
            map.insert(53, &[2052]);     // 53 => [2052]
            map.insert(40, &[264]);     // 40 => [264]
            map.insert(43, &[520]);     // 43 => [520]
            map.insert(45, &[1032]);     // 45 => [1032]
            map.insert(57, &[2056]);     // 57 => [2056]
            map.insert(46, &[1040]);     // 46 => [1040]
            map.insert(58, &[2064]);     // 58 => [2064]
            map.insert(60, &[2080]);     // 60 => [2080]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(14, &[35]);     // 14 => [35]
            map.insert(19, &[67]);     // 19 => [67]
            map.insert(26, &[131]);     // 26 => [131]
            map.insert(38, &[1027]);     // 38 => [1027]
            map.insert(50, &[2051]);     // 50 => [2051]
            map.insert(21, &[69]);     // 21 => [69]
            map.insert(28, &[133]);     // 28 => [133]
            map.insert(52, &[2053]);     // 52 => [2053]
            map.insert(41, &[265]);     // 41 => [265]
            map.insert(42, &[521]);     // 42 => [521]
            map.insert(44, &[1033]);     // 44 => [1033]
            map.insert(56, &[2057]);     // 56 => [2057]
            map.insert(47, &[1041]);     // 47 => [1041]
            map.insert(59, &[2065]);     // 59 => [2065]
            map.insert(61, &[2081]);     // 61 => [2081]
            map.insert(22, &[70]);     // 22 => [70]
            map.insert(31, &[134]);     // 31 => [134]
            map.insert(55, &[2054]);     // 55 => [2054]
            map.insert(62, &[2082]);     // 62 => [2082]
            map.insert(23, &[71]);     // 23 => [71]
            map.insert(30, &[135]);     // 30 => [135]
            map.insert(54, &[2055]);     // 54 => [2055]
            map.insert(63, &[2083]);     // 63 => [2083]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode12_6 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode12_6 {
    fn name(&self) -> String {
        "[12, 6] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        12
    }

    fn dimension(&self) -> usize {
        6
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
        let mut error = BinVector::with_capacity(12);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 12 / 64 + if 12 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(12) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(6);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[12 / 64] & !((1 << 12) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode12_6.generator_matrix();
        assert_eq!(code.ncols(), 12);
        assert_eq!(code.nrows(), 6);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode12_6;
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
            let code = GuavaCode12_6;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, false, true, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, false, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_6;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, false, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, false, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_6;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, false, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_6;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, false, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, false, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_6;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, true, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_6;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_6;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, false, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_6;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, false, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, true, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_6;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, true, false, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, true, true, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_6;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, true, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_6;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, false, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_6;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, false, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_6;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, false, true, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, false, true, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_6;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, true, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, false, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_6;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, true, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_6;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, false, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, false, true, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_6;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_6;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_6;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, true, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_6;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, false, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, false, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, true, false, true, false, false, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
