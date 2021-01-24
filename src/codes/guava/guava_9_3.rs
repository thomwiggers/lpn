use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[9, 3]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode9_3;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 305 ],
                &[ 338 ],
                &[ 404 ],
                
            ], 9));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 449 ],
                &[ 66 ],
                &[ 132 ],
                &[ 8 ],
                &[ 272 ],
                &[ 480 ],
                
            ], 9));
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
            map.insert(35, &[64]);     // 35 => [64]
            map.insert(37, &[128]);     // 37 => [128]
            map.insert(49, &[256]);     // 49 => [256]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(33, &[33]);     // 33 => [33]
            map.insert(34, &[65]);     // 34 => [65]
            map.insert(36, &[129]);     // 36 => [129]
            map.insert(48, &[257]);     // 48 => [257]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(18, &[18]);     // 18 => [18]
            map.insert(39, &[130]);     // 39 => [130]
            map.insert(51, &[258]);     // 51 => [258]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(20, &[20]);     // 20 => [20]
            map.insert(53, &[260]);     // 53 => [260]
            map.insert(24, &[24]);     // 24 => [24]
            map.insert(40, &[40]);     // 40 => [40]
            map.insert(43, &[72]);     // 43 => [72]
            map.insert(45, &[136]);     // 45 => [136]
            map.insert(57, &[264]);     // 57 => [264]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(11, &[11]);     // 11 => [11]
            map.insert(19, &[19]);     // 19 => [19]
            map.insert(38, &[131]);     // 38 => [131]
            map.insert(50, &[259]);     // 50 => [259]
            map.insert(13, &[13]);     // 13 => [13]
            map.insert(21, &[21]);     // 21 => [21]
            map.insert(52, &[261]);     // 52 => [261]
            map.insert(25, &[25]);     // 25 => [25]
            map.insert(41, &[41]);     // 41 => [41]
            map.insert(42, &[73]);     // 42 => [73]
            map.insert(44, &[137]);     // 44 => [137]
            map.insert(56, &[265]);     // 56 => [265]
            map.insert(14, &[14]);     // 14 => [14]
            map.insert(22, &[22]);     // 22 => [22]
            map.insert(55, &[262]);     // 55 => [262]
            map.insert(26, &[26]);     // 26 => [26]
            map.insert(47, &[138]);     // 47 => [138]
            map.insert(59, &[266]);     // 59 => [266]
            map.insert(28, &[28]);     // 28 => [28]
            map.insert(61, &[268]);     // 61 => [268]
            map.insert(15, &[15]);     // 15 => [15]
            map.insert(23, &[23]);     // 23 => [23]
            map.insert(54, &[263]);     // 54 => [263]
            map.insert(27, &[27]);     // 27 => [27]
            map.insert(46, &[139]);     // 46 => [139]
            map.insert(58, &[267]);     // 58 => [267]
            map.insert(29, &[29]);     // 29 => [29]
            map.insert(60, &[269]);     // 60 => [269]
            map.insert(30, &[30]);     // 30 => [30]
            map.insert(63, &[270]);     // 63 => [270]
            map.insert(31, &[31]);     // 31 => [31]
            map.insert(62, &[271]);     // 62 => [271]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode9_3 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode9_3 {
    fn name(&self) -> String {
        "[9, 3] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        9
    }

    fn dimension(&self) -> usize {
        3
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
        let mut error = BinVector::with_capacity(9);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 9 / 64 + if 9 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(9) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(3);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[9 / 64] & !((1 << 9) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode9_3.generator_matrix();
        assert_eq!(code.ncols(), 9);
        assert_eq!(code.nrows(), 3);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode9_3;
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
            let code = GuavaCode9_3;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode9_3;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode9_3;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode9_3;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode9_3;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode9_3;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode9_3;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode9_3;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode9_3;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode9_3;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode9_3;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode9_3;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode9_3;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode9_3;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode9_3;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode9_3;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode9_3;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode9_3;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode9_3;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode9_3;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, true, true, false, false, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
