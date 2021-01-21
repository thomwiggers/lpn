use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[23, 17]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode23_17;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 1703937 ],
                &[ 2752514 ],
                &[ 4849668 ],
                &[ 3276808 ],
                &[ 5373968 ],
                &[ 6422560 ],
                &[ 8126528 ],
                &[ 1835136 ],
                &[ 2883840 ],
                &[ 4981248 ],
                &[ 3408896 ],
                &[ 5507072 ],
                &[ 6557696 ],
                &[ 3678208 ],
                &[ 5783552 ],
                &[ 6848512 ],
                &[ 7405568 ],
                
            ], 23));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 1806113 ],
                &[ 2840210 ],
                &[ 4572596 ],
                &[ 7806008 ],
                &[ 270272 ],
                &[ 7987200 ],
                
            ], 23));
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
            map.insert(13, &[32]);     // 13 => [32]
            map.insert(16, &[64]);     // 16 => [64]
            map.insert(22, &[128]);     // 22 => [128]
            map.insert(21, &[256]);     // 21 => [256]
            map.insert(19, &[512]);     // 19 => [512]
            map.insert(31, &[1024]);     // 31 => [1024]
            map.insert(25, &[2048]);     // 25 => [2048]
            map.insert(26, &[4096]);     // 26 => [4096]
            map.insert(32, &[8192]);     // 32 => [8192]
            map.insert(38, &[16384]);     // 38 => [16384]
            map.insert(37, &[32768]);     // 37 => [32768]
            map.insert(47, &[65536]);     // 47 => [65536]
            map.insert(11, &[131072]);     // 11 => [131072]
            map.insert(28, &[262144]);     // 28 => [262144]
            map.insert(35, &[524288]);     // 35 => [524288]
            map.insert(41, &[1048576]);     // 41 => [1048576]
            map.insert(42, &[2097152]);     // 42 => [2097152]
            map.insert(44, &[4194304]);     // 44 => [4194304]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(15, &[17]);     // 15 => [17]
            map.insert(12, &[33]);     // 12 => [33]
            map.insert(17, &[65]);     // 17 => [65]
            map.insert(23, &[129]);     // 23 => [129]
            map.insert(20, &[257]);     // 20 => [257]
            map.insert(18, &[513]);     // 18 => [513]
            map.insert(30, &[1025]);     // 30 => [1025]
            map.insert(24, &[2049]);     // 24 => [2049]
            map.insert(27, &[4097]);     // 27 => [4097]
            map.insert(33, &[8193]);     // 33 => [8193]
            map.insert(39, &[16385]);     // 39 => [16385]
            map.insert(36, &[32769]);     // 36 => [32769]
            map.insert(46, &[65537]);     // 46 => [65537]
            map.insert(10, &[131073]);     // 10 => [131073]
            map.insert(29, &[262145]);     // 29 => [262145]
            map.insert(34, &[524289]);     // 34 => [524289]
            map.insert(40, &[1048577]);     // 40 => [1048577]
            map.insert(43, &[2097153]);     // 43 => [2097153]
            map.insert(45, &[4194305]);     // 45 => [4194305]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(48, &[8256]);     // 48 => [8256]
            map.insert(54, &[16448]);     // 54 => [16448]
            map.insert(53, &[32832]);     // 53 => [32832]
            map.insert(63, &[65600]);     // 63 => [65600]
            map.insert(51, &[524352]);     // 51 => [524352]
            map.insert(57, &[1048640]);     // 57 => [1048640]
            map.insert(58, &[2097216]);     // 58 => [2097216]
            map.insert(60, &[4194368]);     // 60 => [4194368]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(49, &[8257]);     // 49 => [8257]
            map.insert(55, &[16449]);     // 55 => [16449]
            map.insert(52, &[32833]);     // 52 => [32833]
            map.insert(62, &[65601]);     // 62 => [65601]
            map.insert(50, &[524353]);     // 50 => [524353]
            map.insert(56, &[1048641]);     // 56 => [1048641]
            map.insert(59, &[2097217]);     // 59 => [2097217]
            map.insert(61, &[4194369]);     // 61 => [4194369]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode23_17 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode23_17 {
    fn name(&self) -> String {
        "[23, 17] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        23
    }

    fn dimension(&self) -> usize {
        17
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
        let mut error = BinVector::with_capacity(23);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 23 / 64 + if 23 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(23) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(17);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[23 / 64] & !((1 << 23) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode23_17.generator_matrix();
        assert_eq!(code.ncols(), 23);
        assert_eq!(code.nrows(), 17);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode23_17;
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
            let code = GuavaCode23_17;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, false, false, false, false, true, true, false, false, true, false, false, true, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, true, false, false, true, true, false, false, true, false, false, true, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_17;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, false, true, true, false, true, true, false, true, true, false, true, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, false, true, true, false, true, false, false, true, true, false, true, true, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_17;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, true, false, false, false, true, false, true, true, true, false, false, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, true, false, false, false, true, true, true, true, true, false, false, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_17;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, true, true, false, true, false, false, false, true, true, false, true, true, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, false, true, false, true, false, false, false, true, true, true, true, true, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_17;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, true, true, false, true, true, true, true, false, false, true, false, false, false, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, true, true, false, true, true, false, true, false, false, true, false, false, false, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_17;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, true, false, false, true, false, false, false, true, false, false, false, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_17;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, true, false, false, false, true, true, false, false, false, true, true, true, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, true, true, false, false, false, true, true, false, false, false, true, true, true, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_17;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, true, false, false, true, true, true, false, false, true, true, true, true, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, false, false, false, true, true, true, false, false, true, true, false, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_17;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, true, false, true, true, true, true, false, true, true, false, false, true, true, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, false, true, true, true, true, false, true, false, false, false, true, true, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_17;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, true, true, false, false, false, false, true, false, false, false, false, true, false, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, true, false, false, false, false, true, false, false, false, false, true, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_17;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, true, false, true, false, false, true, true, true, true, true, false, false, true, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, false, true, false, false, true, true, true, true, true, false, false, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_17;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, false, true, false, true, true, false, false, false, false, false, true, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, true, true, true, false, true, true, false, false, false, false, false, true, true, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_17;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, false, true, true, true, false, true, true, false, false, true, true, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, true, true, true, true, false, true, true, false, false, true, true, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_17;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, true, true, true, false, true, true, true, false, true, true, true, false, false, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, true, true, false, true, true, true, false, true, true, true, false, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_17;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, false, true, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, false, true, false, false, false, false, false, false, true, false, false, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_17;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, false, false, false, true, true, true, true, true, false, false, true, false, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, true, false, false, true, true, true, true, true, false, false, true, false, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_17;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, true, false, false, true, true, false, true, true, false, false, true, false, true, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, true, false, true, true, false, true, true, false, false, true, false, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_17;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, false, false, false, true, false, false, true, false, true, false, false, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, true, false, false, true, false, false, true, false, true, true, false, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_17;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, true, true, false, false, false, true, false, true, true, false, false, false, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, true, false, false, false, true, false, true, true, false, false, true, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_17;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, false, false, false, false, false, false, true, true, false, true, false, false, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, true, false, false, false, false, false, true, true, false, true, false, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, true, false, false ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
