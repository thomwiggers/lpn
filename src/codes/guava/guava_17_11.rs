use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[17, 11]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode17_11;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 126977 ],
                &[ 28674 ],
                &[ 45060 ],
                &[ 77832 ],
                &[ 53264 ],
                &[ 86048 ],
                &[ 102464 ],
                &[ 57472 ],
                &[ 90368 ],
                &[ 107008 ],
                &[ 115712 ],
                
            ], 17));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 67433 ],
                &[ 94506 ],
                &[ 111180 ],
                &[ 119920 ],
                &[ 124800 ],
                &[ 2048 ],
                
            ], 17));
            let matrix_t = Box::new(matrix.transposed());
            PARITY_MATRIX = Box::into_raw(matrix);
            PARITY_MATRIX_T = Box::into_raw(matrix_t);

            let mut map = Box::new(FnvHashMap::with_capacity_and_hasher(64, Default::default()));
            map.insert(0, &[0]);     // 0 => [0]
            map.insert(1, &[1]);     // 1 => [1]
            map.insert(2, &[2]);     // 2 => [2]
            map.insert(4, &[4]);     // 4 => [4]
            map.insert(7, &[8]);     // 7 => [8]
            map.insert(8, &[16]);     // 8 => [16]
            map.insert(11, &[32]);     // 11 => [32]
            map.insert(13, &[64]);     // 13 => [64]
            map.insert(16, &[128]);     // 16 => [128]
            map.insert(19, &[256]);     // 19 => [256]
            map.insert(21, &[512]);     // 21 => [512]
            map.insert(25, &[1024]);     // 25 => [1024]
            map.insert(32, &[2048]);     // 32 => [2048]
            map.insert(14, &[4096]);     // 14 => [4096]
            map.insert(22, &[8192]);     // 22 => [8192]
            map.insert(26, &[16384]);     // 26 => [16384]
            map.insert(28, &[32768]);     // 28 => [32768]
            map.insert(31, &[65536]);     // 31 => [65536]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(6, &[9]);     // 6 => [9]
            map.insert(9, &[17]);     // 9 => [17]
            map.insert(10, &[33]);     // 10 => [33]
            map.insert(12, &[65]);     // 12 => [65]
            map.insert(17, &[129]);     // 17 => [129]
            map.insert(18, &[257]);     // 18 => [257]
            map.insert(20, &[513]);     // 20 => [513]
            map.insert(24, &[1025]);     // 24 => [1025]
            map.insert(33, &[2049]);     // 33 => [2049]
            map.insert(15, &[4097]);     // 15 => [4097]
            map.insert(23, &[8193]);     // 23 => [8193]
            map.insert(27, &[16385]);     // 27 => [16385]
            map.insert(29, &[32769]);     // 29 => [32769]
            map.insert(30, &[65537]);     // 30 => [65537]
            map.insert(34, &[2050]);     // 34 => [2050]
            map.insert(36, &[2052]);     // 36 => [2052]
            map.insert(39, &[2056]);     // 39 => [2056]
            map.insert(40, &[2064]);     // 40 => [2064]
            map.insert(43, &[2080]);     // 43 => [2080]
            map.insert(45, &[2112]);     // 45 => [2112]
            map.insert(48, &[2176]);     // 48 => [2176]
            map.insert(51, &[2304]);     // 51 => [2304]
            map.insert(53, &[2560]);     // 53 => [2560]
            map.insert(57, &[3072]);     // 57 => [3072]
            map.insert(46, &[6144]);     // 46 => [6144]
            map.insert(54, &[10240]);     // 54 => [10240]
            map.insert(58, &[18432]);     // 58 => [18432]
            map.insert(60, &[34816]);     // 60 => [34816]
            map.insert(63, &[67584]);     // 63 => [67584]
            map.insert(35, &[2051]);     // 35 => [2051]
            map.insert(37, &[2053]);     // 37 => [2053]
            map.insert(38, &[2057]);     // 38 => [2057]
            map.insert(41, &[2065]);     // 41 => [2065]
            map.insert(42, &[2081]);     // 42 => [2081]
            map.insert(44, &[2113]);     // 44 => [2113]
            map.insert(49, &[2177]);     // 49 => [2177]
            map.insert(50, &[2305]);     // 50 => [2305]
            map.insert(52, &[2561]);     // 52 => [2561]
            map.insert(56, &[3073]);     // 56 => [3073]
            map.insert(47, &[6145]);     // 47 => [6145]
            map.insert(55, &[10241]);     // 55 => [10241]
            map.insert(59, &[18433]);     // 59 => [18433]
            map.insert(61, &[34817]);     // 61 => [34817]
            map.insert(62, &[67585]);     // 62 => [67585]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode17_11 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode17_11 {
    fn name(&self) -> String {
        "[17, 11] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        17
    }

    fn dimension(&self) -> usize {
        11
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
        let mut error = BinVector::with_capacity(17);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 17 / 64 + if 17 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(17) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(11);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[17 / 64] & !((1 << 17) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode17_11.generator_matrix();
        assert_eq!(code.ncols(), 17);
        assert_eq!(code.nrows(), 11);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode17_11;
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
            let code = GuavaCode17_11;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, true, true, true, true, true, false, false, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, true, true, true, true, false, false, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_11;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, false, true, true, true, true, false, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, true, false, false, true, true, true, false, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_11;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, true, true, false, true, false, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, true, true, false, true, false, false, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_11;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, false, false, false, true, false, true, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, false, false, true, false, true, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_11;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, false, false, false, true, true, false, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, true, false, false, false, true, true, false, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_11;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, true, false, true, true, true, false, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, false, true, false, true, false, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_11;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, true, false, false, true, false, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, false, true, false, false, true, false, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_11;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, false, false, false, false, true, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, false, false, false, false, true, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_11;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, false, false, true, true, false, true, true, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, false, true, true, false, true, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_11;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, true, true, true, false, true, false, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, true, true, true, false, true, false, false, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_11;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, true, true, true, false, false, true, true, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, true, true, false, false, true, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_11;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, false, false, true, true, false, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, true, false, false, true, true, false, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_11;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, false, false, false, true, false, true, true, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, false, false, true, false, true, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_11;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, true, true, true, true, false, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, true, true, true, true, false, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_11;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, false, true, true, true, true, false, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, false, false, false, true, true, true, false, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_11;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, true, false, false, true, true, false, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, true, false, false, true, false, false, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_11;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, true, false, true, false, true, false, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, true, false, true, true, true, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_11;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, false, true, false, true, false, true, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, true, true, true, false, true, false, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_11;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, true, false, false, true, false, false, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, false, false, true, false, false, false, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_11;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, false, true, false, true, false, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, false, false, true, false, true, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
