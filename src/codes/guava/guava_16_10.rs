use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[16, 10]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode16_10;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 7169 ],
                &[ 11266 ],
                &[ 19460 ],
                &[ 35848 ],
                &[ 37904 ],
                &[ 42016 ],
                &[ 50240 ],
                &[ 39040 ],
                &[ 43264 ],
                &[ 51712 ],
                
            ], 16));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 8033 ],
                &[ 8482 ],
                &[ 16964 ],
                &[ 35848 ],
                &[ 62576 ],
                &[ 64384 ],
                
            ], 16));
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
            map.insert(19, &[32]);     // 19 => [32]
            map.insert(21, &[64]);     // 21 => [64]
            map.insert(32, &[128]);     // 32 => [128]
            map.insert(35, &[256]);     // 35 => [256]
            map.insert(37, &[512]);     // 37 => [512]
            map.insert(25, &[1024]);     // 25 => [1024]
            map.insert(41, &[2048]);     // 41 => [2048]
            map.insert(49, &[4096]);     // 49 => [4096]
            map.insert(50, &[8192]);     // 50 => [8192]
            map.insert(52, &[16384]);     // 52 => [16384]
            map.insert(56, &[32768]);     // 56 => [32768]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(18, &[33]);     // 18 => [33]
            map.insert(20, &[65]);     // 20 => [65]
            map.insert(33, &[129]);     // 33 => [129]
            map.insert(34, &[257]);     // 34 => [257]
            map.insert(36, &[513]);     // 36 => [513]
            map.insert(24, &[1025]);     // 24 => [1025]
            map.insert(40, &[2049]);     // 40 => [2049]
            map.insert(48, &[4097]);     // 48 => [4097]
            map.insert(51, &[8193]);     // 51 => [8193]
            map.insert(53, &[16385]);     // 53 => [16385]
            map.insert(57, &[32769]);     // 57 => [32769]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(23, &[66]);     // 23 => [66]
            map.insert(39, &[514]);     // 39 => [514]
            map.insert(27, &[1026]);     // 27 => [1026]
            map.insert(43, &[2050]);     // 43 => [2050]
            map.insert(54, &[16386]);     // 54 => [16386]
            map.insert(58, &[32770]);     // 58 => [32770]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(29, &[1028]);     // 29 => [1028]
            map.insert(45, &[2052]);     // 45 => [2052]
            map.insert(60, &[32772]);     // 60 => [32772]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(11, &[11]);     // 11 => [11]
            map.insert(22, &[67]);     // 22 => [67]
            map.insert(38, &[515]);     // 38 => [515]
            map.insert(26, &[1027]);     // 26 => [1027]
            map.insert(42, &[2051]);     // 42 => [2051]
            map.insert(55, &[16387]);     // 55 => [16387]
            map.insert(59, &[32771]);     // 59 => [32771]
            map.insert(13, &[13]);     // 13 => [13]
            map.insert(28, &[1029]);     // 28 => [1029]
            map.insert(44, &[2053]);     // 44 => [2053]
            map.insert(61, &[32773]);     // 61 => [32773]
            map.insert(14, &[14]);     // 14 => [14]
            map.insert(31, &[1030]);     // 31 => [1030]
            map.insert(47, &[2054]);     // 47 => [2054]
            map.insert(62, &[32774]);     // 62 => [32774]
            map.insert(15, &[15]);     // 15 => [15]
            map.insert(30, &[1031]);     // 30 => [1031]
            map.insert(46, &[2055]);     // 46 => [2055]
            map.insert(63, &[32775]);     // 63 => [32775]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode16_10 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode16_10 {
    fn name(&self) -> String {
        "[16, 10] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        16
    }

    fn dimension(&self) -> usize {
        10
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
        let mut error = BinVector::with_capacity(16);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 16 / 64 + if 16 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(16) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(10);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[16 / 64] & !((1 << 16) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode16_10.generator_matrix();
        assert_eq!(code.ncols(), 16);
        assert_eq!(code.nrows(), 10);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode16_10;
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
            let code = GuavaCode16_10;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, true, false, true, false, false, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, false, true, false, false, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_10;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, false, true, true, false, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, false, true, true, false, false, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_10;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, false, false, true, true, false, true, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, false, false, true, true, false, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_10;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, false, false, false, true, false, true, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, false, false, true, false, true, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_10;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, true, false, true, false, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, false, true, false, true, false, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_10;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, false, true, false, true, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, false, false, true, false, true, false, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_10;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, true, true, true, false, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, true, true, true, false, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_10;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, false, false, true, false, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, true, false, false, true, false, true, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_10;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, false, true, true, true, true, true, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, false, true, true, true, true, true, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_10;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_10;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, true, true, true, false, false, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, false, true, true, false, false, false, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_10;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, true, true, true, true, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, true, true, true, true, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_10;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, true, false, true, false, true, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, true, false, true, false, true, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_10;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, false, false, false, false, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, false, false, false, false, false, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_10;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, true, false, true, false, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, true, false, true, false, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_10;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, false, true, false, true, false, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, false, true, false, true, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_10;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, true, false, false, true, false, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, true, true, false, false, true, false, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_10;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, false, true, false, true, false, true, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, true, false, true, false, true, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_10;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, true, true, false, false, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, true, true, true, false, false, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_10;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, true, false, true, true, true, true, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, false, true, true, true, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, false, true, true, true, false, false, false ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
