use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[14, 8]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode14_8;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 4865 ],
                &[ 8962 ],
                &[ 9476 ],
                &[ 10504 ],
                &[ 12560 ],
                &[ 9760 ],
                &[ 10816 ],
                &[ 12928 ],
                
            ], 14));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 7937 ],
                &[ 8962 ],
                &[ 15044 ],
                &[ 2120 ],
                &[ 3984 ],
                &[ 16096 ],
                
            ], 14));
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
            map.insert(32, &[32]);     // 32 => [32]
            map.insert(44, &[64]);     // 44 => [64]
            map.insert(52, &[128]);     // 52 => [128]
            map.insert(19, &[256]);     // 19 => [256]
            map.insert(55, &[512]);     // 55 => [512]
            map.insert(49, &[1024]);     // 49 => [1024]
            map.insert(61, &[2048]);     // 61 => [2048]
            map.insert(37, &[4096]);     // 37 => [4096]
            map.insert(38, &[8192]);     // 38 => [8192]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(33, &[33]);     // 33 => [33]
            map.insert(45, &[65]);     // 45 => [65]
            map.insert(53, &[129]);     // 53 => [129]
            map.insert(18, &[257]);     // 18 => [257]
            map.insert(54, &[513]);     // 54 => [513]
            map.insert(48, &[1025]);     // 48 => [1025]
            map.insert(60, &[2049]);     // 60 => [2049]
            map.insert(36, &[4097]);     // 36 => [4097]
            map.insert(39, &[8193]);     // 39 => [8193]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(34, &[34]);     // 34 => [34]
            map.insert(46, &[66]);     // 46 => [66]
            map.insert(51, &[1026]);     // 51 => [1026]
            map.insert(63, &[2050]);     // 63 => [2050]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(20, &[20]);     // 20 => [20]
            map.insert(40, &[68]);     // 40 => [68]
            map.insert(23, &[260]);     // 23 => [260]
            map.insert(57, &[2052]);     // 57 => [2052]
            map.insert(24, &[24]);     // 24 => [24]
            map.insert(27, &[264]);     // 27 => [264]
            map.insert(29, &[2080]);     // 29 => [2080]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(11, &[11]);     // 11 => [11]
            map.insert(35, &[35]);     // 35 => [35]
            map.insert(47, &[67]);     // 47 => [67]
            map.insert(50, &[1027]);     // 50 => [1027]
            map.insert(62, &[2051]);     // 62 => [2051]
            map.insert(13, &[13]);     // 13 => [13]
            map.insert(21, &[21]);     // 21 => [21]
            map.insert(41, &[69]);     // 41 => [69]
            map.insert(22, &[261]);     // 22 => [261]
            map.insert(56, &[2053]);     // 56 => [2053]
            map.insert(25, &[25]);     // 25 => [25]
            map.insert(26, &[265]);     // 26 => [265]
            map.insert(28, &[2081]);     // 28 => [2081]
            map.insert(14, &[14]);     // 14 => [14]
            map.insert(42, &[70]);     // 42 => [70]
            map.insert(59, &[2054]);     // 59 => [2054]
            map.insert(31, &[2082]);     // 31 => [2082]
            map.insert(15, &[15]);     // 15 => [15]
            map.insert(43, &[71]);     // 43 => [71]
            map.insert(58, &[2055]);     // 58 => [2055]
            map.insert(30, &[2083]);     // 30 => [2083]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode14_8 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode14_8 {
    fn name(&self) -> String {
        "[14, 8] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        14
    }

    fn dimension(&self) -> usize {
        8
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
        let mut error = BinVector::with_capacity(14);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 14 / 64 + if 14 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(14) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(8);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[14 / 64] & !((1 << 14) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode14_8.generator_matrix();
        assert_eq!(code.ncols(), 14);
        assert_eq!(code.nrows(), 8);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode14_8;
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
            let code = GuavaCode14_8;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, true, true, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, true, true, true, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_8;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, false, false, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, true, false, false, true, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_8;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, true, false, false, false, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, false, false, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_8;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, true, false, true, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, true, true, false, true, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_8;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, true, false, true, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, false, true, false, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_8;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, true, true, false, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, false, true, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_8;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, true, false, true, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, true, false, true, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_8;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, false, true, false, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, true, true, true, false, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_8;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, false, true, true, true, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, false, true, true, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_8;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, false, true, false, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, true, true, true, false, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_8;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, false, false, true, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, false, false, false, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_8;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, true, true, true, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, true, true, false, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_8;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, false, false, false, false, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, false, false, false, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_8;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, false, false, false, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, false, false, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_8;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, false, true, false, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, false, false, true, false, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_8;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, true, false, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, true, false, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_8;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, true, false, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, false, true, false, true, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_8;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, false, true, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, true, false, true, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_8;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, true, true, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, true, true, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode14_8;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, false, false, false, true, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, false, false, true, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, true, true, false, false, true, false ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
