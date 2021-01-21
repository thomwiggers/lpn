use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[21, 15]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode21_15;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 1212417 ],
                &[ 819202 ],
                &[ 1343492 ],
                &[ 1605640 ],
                &[ 2031632 ],
                &[ 458784 ],
                &[ 720960 ],
                &[ 1245312 ],
                &[ 852224 ],
                &[ 1376768 ],
                &[ 1639424 ],
                &[ 919552 ],
                &[ 1445888 ],
                &[ 1712128 ],
                &[ 1851392 ],
                
            ], 21));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 1918721 ],
                &[ 1175906 ],
                &[ 710052 ],
                &[ 451528 ],
                &[ 67568 ],
                &[ 1996800 ],
                
            ], 21));
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
            map.insert(28, &[128]);     // 28 => [128]
            map.insert(31, &[256]);     // 31 => [256]
            map.insert(25, &[512]);     // 25 => [512]
            map.insert(21, &[1024]);     // 21 => [1024]
            map.insert(32, &[2048]);     // 32 => [2048]
            map.insert(38, &[4096]);     // 38 => [4096]
            map.insert(42, &[8192]);     // 42 => [8192]
            map.insert(47, &[16384]);     // 47 => [16384]
            map.insert(14, &[32768]);     // 14 => [32768]
            map.insert(19, &[65536]);     // 19 => [65536]
            map.insert(44, &[131072]);     // 44 => [131072]
            map.insert(41, &[262144]);     // 41 => [262144]
            map.insert(37, &[524288]);     // 37 => [524288]
            map.insert(35, &[1048576]);     // 35 => [1048576]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(23, &[33]);     // 23 => [33]
            map.insert(27, &[65]);     // 27 => [65]
            map.insert(29, &[129]);     // 29 => [129]
            map.insert(30, &[257]);     // 30 => [257]
            map.insert(24, &[513]);     // 24 => [513]
            map.insert(20, &[1025]);     // 20 => [1025]
            map.insert(33, &[2049]);     // 33 => [2049]
            map.insert(39, &[4097]);     // 39 => [4097]
            map.insert(43, &[8193]);     // 43 => [8193]
            map.insert(46, &[16385]);     // 46 => [16385]
            map.insert(15, &[32769]);     // 15 => [32769]
            map.insert(18, &[65537]);     // 18 => [65537]
            map.insert(45, &[131073]);     // 45 => [131073]
            map.insert(40, &[262145]);     // 40 => [262145]
            map.insert(36, &[524289]);     // 36 => [524289]
            map.insert(34, &[1048577]);     // 34 => [1048577]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(12, &[32770]);     // 12 => [32770]
            map.insert(48, &[2064]);     // 48 => [2064]
            map.insert(54, &[4112]);     // 54 => [4112]
            map.insert(58, &[8208]);     // 58 => [8208]
            map.insert(63, &[16400]);     // 63 => [16400]
            map.insert(60, &[131088]);     // 60 => [131088]
            map.insert(57, &[262160]);     // 57 => [262160]
            map.insert(53, &[524304]);     // 53 => [524304]
            map.insert(51, &[1048592]);     // 51 => [1048592]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(11, &[11]);     // 11 => [11]
            map.insert(13, &[32771]);     // 13 => [32771]
            map.insert(49, &[2065]);     // 49 => [2065]
            map.insert(55, &[4113]);     // 55 => [4113]
            map.insert(59, &[8209]);     // 59 => [8209]
            map.insert(62, &[16401]);     // 62 => [16401]
            map.insert(61, &[131089]);     // 61 => [131089]
            map.insert(56, &[262161]);     // 56 => [262161]
            map.insert(52, &[524305]);     // 52 => [524305]
            map.insert(50, &[1048593]);     // 50 => [1048593]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode21_15 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode21_15 {
    fn name(&self) -> String {
        "[21, 15] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        21
    }

    fn dimension(&self) -> usize {
        15
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
        let mut error = BinVector::with_capacity(21);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 21 / 64 + if 21 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(21) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(15);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[21 / 64] & !((1 << 21) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode21_15.generator_matrix();
        assert_eq!(code.ncols(), 21);
        assert_eq!(code.nrows(), 15);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode21_15;
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
            let code = GuavaCode21_15;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, true, true, true, false, false, true, true, true, false, true, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, true, true, false, false, true, true, true, false, true, true, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_15;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, true, false, true, true, false, false, true, true, true, true, false, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, false, true, true, true, false, true, true, true, true, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_15;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, false, true, false, false, false, true, false, false, true, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, false, true, false, false, false, true, false, false, true, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_15;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, true, false, false, false, true, true, false, false, true, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, false, false, false, false, true, true, false, false, true, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_15;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, true, false, false, true, true, true, true, true, false, false, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, true, false, false, true, true, true, true, true, false, true, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_15;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, true, false, true, true, true, true, true, false, false, true, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, true, false, true, true, true, true, true, false, false, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_15;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, false, true, false, false, true, true, true, false, false, false, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, true, false, false, true, true, true, false, false, false, true, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_15;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, false, false, true, true, false, false, true, false, false, false, true, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, false, true, true, false, false, true, false, false, false, true, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_15;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, true, true, true, false, false, false, false, true, false, false, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, true, true, false, false, false, false, true, false, true, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_15;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, false, true, false, false, false, false, true, false, false, true, true, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, false, true, false, false, false, false, true, true, false, true, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_15;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, false, false, true, true, true, true, true, false, false, true, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, false, false, true, true, true, true, true, false, false, true, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_15;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, true, false, false, true, false, false, false, false, false, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, false, false, false, true, false, false, false, false, false, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_15;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, false, false, true, false, true, false, true, false, true, false, true, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, false, true, false, true, false, true, false, true, false, true, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_15;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, true, true, false, true, true, false, true, true, true, false, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, true, true, false, true, true, false, false, true, true, false, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_15;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, false, true, false, true, true, true, true, false, true, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, false, false, true, false, true, true, true, true, false, true, true, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_15;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, false, true, false, false, true, true, true, true, true, false, true, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, false, true, false, false, true, true, true, true, true, false, true, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_15;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, true, false, true, false, false, true, false, true, false, true, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, true, false, true, false, false, true, false, true, false, true, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_15;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, true, false, false, false, true, true, false, false, true, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, false, true, false, false, false, true, true, false, false, false, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_15;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, true, true, false, true, true, false, false, true, false, false, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, false, true, true, false, true, true, false, false, true, false, false, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_15;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, true, true, false, false, false, false, true, false, false, true, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, true, false, false, false, false, true, false, false, true, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
