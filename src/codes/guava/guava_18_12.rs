use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[18, 12]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode18_12;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 200705 ],
                &[ 253954 ],
                &[ 57348 ],
                &[ 90120 ],
                &[ 155664 ],
                &[ 106528 ],
                &[ 172096 ],
                &[ 204928 ],
                &[ 114944 ],
                &[ 180736 ],
                &[ 214016 ],
                &[ 231424 ],
                
            ], 18));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 4097 ],
                &[ 138962 ],
                &[ 193108 ],
                &[ 222360 ],
                &[ 239840 ],
                &[ 249600 ],
                
            ], 18));
            let matrix_t = Box::new(matrix.transposed());
            PARITY_MATRIX = Box::into_raw(matrix);
            PARITY_MATRIX_T = Box::into_raw(matrix_t);

            let mut map = Box::new(FnvHashMap::with_capacity_and_hasher(64, Default::default()));
            map.insert(0, &[0]);     // 0 => [0]
            map.insert(1, &[1]);     // 1 => [1]
            map.insert(2, &[2]);     // 2 => [2]
            map.insert(4, &[4]);     // 4 => [4]
            map.insert(8, &[8]);     // 8 => [8]
            map.insert(14, &[16]);     // 14 => [16]
            map.insert(16, &[32]);     // 16 => [32]
            map.insert(22, &[64]);     // 22 => [64]
            map.insert(26, &[128]);     // 26 => [128]
            map.insert(32, &[256]);     // 32 => [256]
            map.insert(38, &[512]);     // 38 => [512]
            map.insert(42, &[1024]);     // 42 => [1024]
            map.insert(50, &[2048]);     // 50 => [2048]
            map.insert(7, &[4096]);     // 7 => [4096]
            map.insert(28, &[8192]);     // 28 => [8192]
            map.insert(44, &[16384]);     // 44 => [16384]
            map.insert(52, &[32768]);     // 52 => [32768]
            map.insert(56, &[65536]);     // 56 => [65536]
            map.insert(62, &[131072]);     // 62 => [131072]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(15, &[17]);     // 15 => [17]
            map.insert(17, &[33]);     // 17 => [33]
            map.insert(23, &[65]);     // 23 => [65]
            map.insert(27, &[129]);     // 27 => [129]
            map.insert(33, &[257]);     // 33 => [257]
            map.insert(39, &[513]);     // 39 => [513]
            map.insert(43, &[1025]);     // 43 => [1025]
            map.insert(51, &[2049]);     // 51 => [2049]
            map.insert(6, &[4097]);     // 6 => [4097]
            map.insert(29, &[8193]);     // 29 => [8193]
            map.insert(45, &[16385]);     // 45 => [16385]
            map.insert(53, &[32769]);     // 53 => [32769]
            map.insert(57, &[65537]);     // 57 => [65537]
            map.insert(63, &[131073]);     // 63 => [131073]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(12, &[18]);     // 12 => [18]
            map.insert(18, &[34]);     // 18 => [34]
            map.insert(20, &[66]);     // 20 => [66]
            map.insert(24, &[130]);     // 24 => [130]
            map.insert(34, &[258]);     // 34 => [258]
            map.insert(36, &[514]);     // 36 => [514]
            map.insert(40, &[1026]);     // 40 => [1026]
            map.insert(48, &[2050]);     // 48 => [2050]
            map.insert(30, &[8194]);     // 30 => [8194]
            map.insert(46, &[16386]);     // 46 => [16386]
            map.insert(54, &[32770]);     // 54 => [32770]
            map.insert(58, &[65538]);     // 58 => [65538]
            map.insert(60, &[131074]);     // 60 => [131074]
            map.insert(11, &[11]);     // 11 => [11]
            map.insert(13, &[19]);     // 13 => [19]
            map.insert(19, &[35]);     // 19 => [35]
            map.insert(21, &[67]);     // 21 => [67]
            map.insert(25, &[131]);     // 25 => [131]
            map.insert(35, &[259]);     // 35 => [259]
            map.insert(37, &[515]);     // 37 => [515]
            map.insert(41, &[1027]);     // 41 => [1027]
            map.insert(49, &[2051]);     // 49 => [2051]
            map.insert(31, &[8195]);     // 31 => [8195]
            map.insert(47, &[16387]);     // 47 => [16387]
            map.insert(55, &[32771]);     // 55 => [32771]
            map.insert(59, &[65539]);     // 59 => [65539]
            map.insert(61, &[131075]);     // 61 => [131075]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode18_12 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode18_12 {
    fn name(&self) -> String {
        "[18, 12] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        18
    }

    fn dimension(&self) -> usize {
        12
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
        let mut error = BinVector::with_capacity(18);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 18 / 64 + if 18 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(18) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(12);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[18 / 64] & !((1 << 18) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode18_12.generator_matrix();
        assert_eq!(code.ncols(), 18);
        assert_eq!(code.nrows(), 12);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode18_12;
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
            let code = GuavaCode18_12;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, true, true, false, false, false, false, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, true, true, false, false, false, true, false, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_12;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, true, true, true, true, false, true, true, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, true, true, false, true, false, true, true, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_12;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, false, true, true, true, false, true, false, true, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, true, true, true, false, true, false, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_12;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, true, false, true, false, true, false, true, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, true, false, false, false, true, false, true, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_12;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, true, true, false, false, false, true, true, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, true, true, false, false, false, true, true, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_12;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, true, false, true, true, true, false, true, true, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, true, false, true, true, true, false, true, true, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_12;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, false, false, false, true, true, false, false, false, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_12;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, true, true, true, true, false, true, false, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, true, true, true, false, true, false, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_12;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, false, true, true, false, true, false, false, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, false, true, true, false, true, false, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_12;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, false, false, true, true, true, true, false, true, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, false, false, true, true, true, true, false, true, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_12;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, true, false, true, false, true, true, false, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, true, false, false, false, true, true, false, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_12;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, true, true, false, true, true, true, false, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, true, false, true, true, true, false, false, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_12;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, false, false, true, false, true, false, true, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, false, false, false, false, true, false, true, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_12;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, true, true, true, true, false, true, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, true, false, true, true, false, true, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_12;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_12;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, false, true, false, true, true, false, true, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, false, true, false, true, true, false, true, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_12;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, false, true, false, true, false, true, false, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, true, false, true, false, true, true, false, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_12;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, true, false, false, true, true, true, false, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, false, false, true, true, false, false, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_12;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, true, true, true, true, true, true, true, false, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, true, true, false, true, true, true, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_12;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, true, true, false, false, false, true, true, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, false, true, true, false, false, false, true, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
