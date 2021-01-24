use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[13, 7]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode13_7;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 4481 ],
                &[ 4738 ],
                &[ 5252 ],
                &[ 6280 ],
                &[ 4880 ],
                &[ 5408 ],
                &[ 6464 ],
                
            ], 13));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 7681 ],
                &[ 7522 ],
                &[ 1060 ],
                &[ 2120 ],
                &[ 8048 ],
                &[ 3968 ],
                
            ], 13));
            let matrix_t = Box::new(matrix.transposed());
            PARITY_MATRIX = Box::into_raw(matrix);
            PARITY_MATRIX_T = Box::into_raw(matrix_t);

            let mut map = Box::new(FnvHashMap::with_capacity_and_hasher(64, Default::default()));
            map.insert(0, &[0]);     // 0 => [0]
            map.insert(1, &[1]);     // 1 => [1]
            map.insert(2, &[2]);     // 2 => [2]
            map.insert(4, &[4]);     // 4 => [4]
            map.insert(8, &[8]);     // 8 => [8]
            map.insert(16, &[16]);     // 16 => [16]
            map.insert(22, &[32]);     // 22 => [32]
            map.insert(26, &[64]);     // 26 => [64]
            map.insert(32, &[128]);     // 32 => [128]
            map.insert(50, &[256]);     // 50 => [256]
            map.insert(49, &[512]);     // 49 => [512]
            map.insert(55, &[1024]);     // 55 => [1024]
            map.insert(59, &[2048]);     // 59 => [2048]
            map.insert(19, &[4096]);     // 19 => [4096]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(23, &[33]);     // 23 => [33]
            map.insert(27, &[65]);     // 27 => [65]
            map.insert(33, &[129]);     // 33 => [129]
            map.insert(51, &[257]);     // 51 => [257]
            map.insert(48, &[513]);     // 48 => [513]
            map.insert(54, &[1025]);     // 54 => [1025]
            map.insert(58, &[2049]);     // 58 => [2049]
            map.insert(18, &[4097]);     // 18 => [4097]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(20, &[34]);     // 20 => [34]
            map.insert(24, &[66]);     // 24 => [66]
            map.insert(34, &[130]);     // 34 => [130]
            map.insert(53, &[1026]);     // 53 => [1026]
            map.insert(57, &[2050]);     // 57 => [2050]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(30, &[68]);     // 30 => [68]
            map.insert(36, &[132]);     // 36 => [132]
            map.insert(63, &[2052]);     // 63 => [2052]
            map.insert(40, &[136]);     // 40 => [136]
            map.insert(39, &[1040]);     // 39 => [1040]
            map.insert(43, &[2064]);     // 43 => [2064]
            map.insert(45, &[2080]);     // 45 => [2080]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(11, &[11]);     // 11 => [11]
            map.insert(21, &[35]);     // 21 => [35]
            map.insert(25, &[67]);     // 25 => [67]
            map.insert(35, &[131]);     // 35 => [131]
            map.insert(52, &[1027]);     // 52 => [1027]
            map.insert(56, &[2051]);     // 56 => [2051]
            map.insert(13, &[13]);     // 13 => [13]
            map.insert(31, &[69]);     // 31 => [69]
            map.insert(37, &[133]);     // 37 => [133]
            map.insert(62, &[2053]);     // 62 => [2053]
            map.insert(41, &[137]);     // 41 => [137]
            map.insert(38, &[1041]);     // 38 => [1041]
            map.insert(42, &[2065]);     // 42 => [2065]
            map.insert(44, &[2081]);     // 44 => [2081]
            map.insert(14, &[14]);     // 14 => [14]
            map.insert(28, &[70]);     // 28 => [70]
            map.insert(61, &[2054]);     // 61 => [2054]
            map.insert(47, &[2082]);     // 47 => [2082]
            map.insert(15, &[15]);     // 15 => [15]
            map.insert(29, &[71]);     // 29 => [71]
            map.insert(60, &[2055]);     // 60 => [2055]
            map.insert(46, &[2083]);     // 46 => [2083]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode13_7 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode13_7 {
    fn name(&self) -> String {
        "[13, 7] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        13
    }

    fn dimension(&self) -> usize {
        7
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
        let mut error = BinVector::with_capacity(13);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 13 / 64 + if 13 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(13) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(7);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[13 / 64] & !((1 << 13) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode13_7.generator_matrix();
        assert_eq!(code.ncols(), 13);
        assert_eq!(code.nrows(), 7);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode13_7;
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
            let code = GuavaCode13_7;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, true, true, false, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, true, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_7;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, false, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, true, false, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_7;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, true, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, true, false, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_7;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, true, false, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, false, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_7;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, false, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, false, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_7;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, false, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, false, false, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_7;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, true, false, true, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, false, true, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_7;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, false, false, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, false, false, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_7;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, false, false, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, false, false, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_7;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, true, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, false, true, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_7;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, true, false, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, true, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_7;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, true, true, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, false, true, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_7;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, false, false, false, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, false, false, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_7;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, false, false, false, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, false, true, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_7;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, false, false, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, false, false, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_7;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, true, false, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, true, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_7;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, true, true, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, true, true, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_7;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, true, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, false, false, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_7;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, false, true, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, false, true, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_7;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, false, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, false, false, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, true, true, false, false, false, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
