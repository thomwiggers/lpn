use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[24, 18]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode24_18;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 15990785 ],
                &[ 3407874 ],
                &[ 5505028 ],
                &[ 9699336 ],
                &[ 6553616 ],
                &[ 10747936 ],
                &[ 12845120 ],
                &[ 16253056 ],
                &[ 3670272 ],
                &[ 5767680 ],
                &[ 9962496 ],
                &[ 6817792 ],
                &[ 11014144 ],
                &[ 13115392 ],
                &[ 7356416 ],
                &[ 11567104 ],
                &[ 13697024 ],
                &[ 14811136 ],
                
            ], 24));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 9145193 ],
                &[ 12358954 ],
                &[ 14493260 ],
                &[ 15612016 ],
                &[ 540544 ],
                &[ 15974400 ],
                
            ], 24));
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
            map.insert(22, &[1024]);     // 22 => [1024]
            map.insert(25, &[2048]);     // 25 => [2048]
            map.insert(26, &[4096]);     // 26 => [4096]
            map.insert(28, &[8192]);     // 28 => [8192]
            map.insert(32, &[16384]);     // 32 => [16384]
            map.insert(35, &[32768]);     // 35 => [32768]
            map.insert(37, &[65536]);     // 37 => [65536]
            map.insert(41, &[131072]);     // 41 => [131072]
            map.insert(14, &[262144]);     // 14 => [262144]
            map.insert(31, &[524288]);     // 31 => [524288]
            map.insert(38, &[1048576]);     // 38 => [1048576]
            map.insert(42, &[2097152]);     // 42 => [2097152]
            map.insert(44, &[4194304]);     // 44 => [4194304]
            map.insert(47, &[8388608]);     // 47 => [8388608]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(6, &[9]);     // 6 => [9]
            map.insert(9, &[17]);     // 9 => [17]
            map.insert(10, &[33]);     // 10 => [33]
            map.insert(12, &[65]);     // 12 => [65]
            map.insert(17, &[129]);     // 17 => [129]
            map.insert(18, &[257]);     // 18 => [257]
            map.insert(20, &[513]);     // 20 => [513]
            map.insert(23, &[1025]);     // 23 => [1025]
            map.insert(24, &[2049]);     // 24 => [2049]
            map.insert(27, &[4097]);     // 27 => [4097]
            map.insert(29, &[8193]);     // 29 => [8193]
            map.insert(33, &[16385]);     // 33 => [16385]
            map.insert(34, &[32769]);     // 34 => [32769]
            map.insert(36, &[65537]);     // 36 => [65537]
            map.insert(40, &[131073]);     // 40 => [131073]
            map.insert(15, &[262145]);     // 15 => [262145]
            map.insert(30, &[524289]);     // 30 => [524289]
            map.insert(39, &[1048577]);     // 39 => [1048577]
            map.insert(43, &[2097153]);     // 43 => [2097153]
            map.insert(45, &[4194305]);     // 45 => [4194305]
            map.insert(46, &[8388609]);     // 46 => [8388609]
            map.insert(48, &[16512]);     // 48 => [16512]
            map.insert(51, &[32896]);     // 51 => [32896]
            map.insert(53, &[65664]);     // 53 => [65664]
            map.insert(57, &[131200]);     // 57 => [131200]
            map.insert(54, &[1048704]);     // 54 => [1048704]
            map.insert(58, &[2097280]);     // 58 => [2097280]
            map.insert(60, &[4194432]);     // 60 => [4194432]
            map.insert(63, &[8388736]);     // 63 => [8388736]
            map.insert(49, &[16513]);     // 49 => [16513]
            map.insert(50, &[32897]);     // 50 => [32897]
            map.insert(52, &[65665]);     // 52 => [65665]
            map.insert(56, &[131201]);     // 56 => [131201]
            map.insert(55, &[1048705]);     // 55 => [1048705]
            map.insert(59, &[2097281]);     // 59 => [2097281]
            map.insert(61, &[4194433]);     // 61 => [4194433]
            map.insert(62, &[8388737]);     // 62 => [8388737]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode24_18 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode24_18 {
    fn name(&self) -> String {
        "[24, 18] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        24
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
        codeword.truncate(18);
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
        let code = GuavaCode24_18.generator_matrix();
        assert_eq!(code.ncols(), 24);
        assert_eq!(code.nrows(), 18);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode24_18;
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
            let code = GuavaCode24_18;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, true, false, false, true, true, false, false, true, true, true, false, false, false, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, true, false, false, true, true, false, false, true, true, true, false, false, false, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_18;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, false, false, false, false, false, false, false, false, true, true, true, true, false, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, false, false, true, false, false, false, false, false, false, true, true, true, true, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_18;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, false, false, false, false, false, true, true, false, false, false, true, false, true, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, true, false, false, false, false, true, true, false, false, false, true, false, true, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_18;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, false, true, true, false, true, true, false, true, true, false, true, false, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, false, true, true, false, true, true, false, true, true, false, true, false, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_18;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, false, false, false, false, true, false, true, true, false, false, false, false, true, true, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, false, false, false, false, true, false, true, true, true, false, false, false, true, true, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_18;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, false, true, true, false, false, false, false, true, false, false, false, false, false, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, false, false, true, true, false, false, false, false, true, true, false, false, false, false, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_18;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, true, false, true, false, true, false, true, true, true, false, true, false, false, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, true, true, true, false, true, false, true, true, true, false, true, false, false, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_18;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, true, false, true, true, false, false, true, true, true, true, false, false, false, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, true, false, true, true, false, false, true, true, true, true, true, false, false, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_18;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, true, false, false, false, false, true, false, true, false, true, true, false, true, false, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, true, false, false, false, false, true, false, true, true, true, true, false, true, false, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_18;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, true, true, true, true, true, true, false, false, true, true, true, false, false, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, true, true, true, true, true, true, false, false, true, true, true, false, false, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_18;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, true, false, false, true, false, true, true, true, true, false, false, true, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, true, false, false, true, false, true, true, true, true, false, false, true, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_18;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, true, false, false, true, false, false, false, false, true, false, true, false, true, true, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, true, false, false, true, false, false, true, false, true, false, true, false, true, true, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_18;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, false, false, false, false, true, true, true, true, false, false, true, false, true, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, false, false, true, false, true, true, true, true, false, false, true, false, true, true, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_18;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, false, false, false, false, true, true, true, true, false, false, false, true, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, true, false, false, false, false, true, false, true, true, false, false, false, true, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_18;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, false, true, true, false, true, false, true, false, true, false, false, false, true, false, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, true, true, true, true, false, true, false, true, false, false, false, true, false, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_18;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, true, true, false, true, false, true, false, true, false, false, true, true, false, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, true, true, true, false, true, false, true, false, false, true, true, false, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_18;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, true, false, true, false, true, false, false, true, true, true, false, false, true, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, false, true, false, true, false, true, false, false, true, true, true, false, false, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_18;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, false, true, true, false, true, false, false, false, false, false, false, true, true, true, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, true, true, false, true, false, false, false, true, false, false, true, true, true, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_18;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, false, true, false, false, true, true, true, true, true, false, false, true, true, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, true, false, false, false, false, true, true, true, true, true, false, false, true, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_18;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, true, false, true, false, true, true, false, false, true, false, false, true, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, false, true, false, true, false, true, true, false, false, true, false, false, true, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, true, true, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
