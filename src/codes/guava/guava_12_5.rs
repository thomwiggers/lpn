use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[12, 5]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode12_5;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 2337 ],
                &[ 2594 ],
                &[ 3108 ],
                &[ 1352 ],
                &[ 1616 ],
                
            ], 12));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 3665 ],
                &[ 530 ],
                &[ 1092 ],
                &[ 88 ],
                &[ 2080 ],
                &[ 128 ],
                &[ 3840 ],
                
            ], 12));
            let matrix_t = Box::new(matrix.transposed());
            PARITY_MATRIX = Box::into_raw(matrix);
            PARITY_MATRIX_T = Box::into_raw(matrix_t);

            let mut map = Box::new(FnvHashMap::with_capacity_and_hasher(128, Default::default()));
            map.insert(0, &[0]);     // 0 => [0]
            map.insert(1, &[1]);     // 1 => [1]
            map.insert(2, &[2]);     // 2 => [2]
            map.insert(4, &[4]);     // 4 => [4]
            map.insert(8, &[8]);     // 8 => [8]
            map.insert(11, &[16]);     // 11 => [16]
            map.insert(16, &[32]);     // 16 => [32]
            map.insert(13, &[64]);     // 13 => [64]
            map.insert(32, &[128]);     // 32 => [128]
            map.insert(64, &[256]);     // 64 => [256]
            map.insert(67, &[512]);     // 67 => [512]
            map.insert(69, &[1024]);     // 69 => [1024]
            map.insert(81, &[2048]);     // 81 => [2048]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(10, &[17]);     // 10 => [17]
            map.insert(17, &[33]);     // 17 => [33]
            map.insert(12, &[65]);     // 12 => [65]
            map.insert(33, &[129]);     // 33 => [129]
            map.insert(65, &[257]);     // 65 => [257]
            map.insert(66, &[513]);     // 66 => [513]
            map.insert(68, &[1025]);     // 68 => [1025]
            map.insert(80, &[2049]);     // 80 => [2049]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(18, &[34]);     // 18 => [34]
            map.insert(15, &[66]);     // 15 => [66]
            map.insert(34, &[130]);     // 34 => [130]
            map.insert(71, &[1026]);     // 71 => [1026]
            map.insert(83, &[2050]);     // 83 => [2050]
            map.insert(20, &[36]);     // 20 => [36]
            map.insert(36, &[132]);     // 36 => [132]
            map.insert(85, &[2052]);     // 85 => [2052]
            map.insert(24, &[40]);     // 24 => [40]
            map.insert(40, &[136]);     // 40 => [136]
            map.insert(72, &[264]);     // 72 => [264]
            map.insert(75, &[520]);     // 75 => [520]
            map.insert(77, &[1032]);     // 77 => [1032]
            map.insert(89, &[2056]);     // 89 => [2056]
            map.insert(27, &[48]);     // 27 => [48]
            map.insert(43, &[144]);     // 43 => [144]
            map.insert(78, &[1040]);     // 78 => [1040]
            map.insert(90, &[2064]);     // 90 => [2064]
            map.insert(29, &[96]);     // 29 => [96]
            map.insert(48, &[160]);     // 48 => [160]
            map.insert(45, &[192]);     // 45 => [192]
            map.insert(92, &[2112]);     // 92 => [2112]
            map.insert(96, &[384]);     // 96 => [384]
            map.insert(99, &[640]);     // 99 => [640]
            map.insert(101, &[1152]);     // 101 => [1152]
            map.insert(113, &[2176]);     // 113 => [2176]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(19, &[35]);     // 19 => [35]
            map.insert(14, &[67]);     // 14 => [67]
            map.insert(35, &[131]);     // 35 => [131]
            map.insert(70, &[1027]);     // 70 => [1027]
            map.insert(82, &[2051]);     // 82 => [2051]
            map.insert(21, &[37]);     // 21 => [37]
            map.insert(37, &[133]);     // 37 => [133]
            map.insert(84, &[2053]);     // 84 => [2053]
            map.insert(25, &[41]);     // 25 => [41]
            map.insert(41, &[137]);     // 41 => [137]
            map.insert(73, &[265]);     // 73 => [265]
            map.insert(74, &[521]);     // 74 => [521]
            map.insert(76, &[1033]);     // 76 => [1033]
            map.insert(88, &[2057]);     // 88 => [2057]
            map.insert(26, &[49]);     // 26 => [49]
            map.insert(42, &[145]);     // 42 => [145]
            map.insert(79, &[1041]);     // 79 => [1041]
            map.insert(91, &[2065]);     // 91 => [2065]
            map.insert(28, &[97]);     // 28 => [97]
            map.insert(49, &[161]);     // 49 => [161]
            map.insert(44, &[193]);     // 44 => [193]
            map.insert(93, &[2113]);     // 93 => [2113]
            map.insert(97, &[385]);     // 97 => [385]
            map.insert(98, &[641]);     // 98 => [641]
            map.insert(100, &[1153]);     // 100 => [1153]
            map.insert(112, &[2177]);     // 112 => [2177]
            map.insert(22, &[38]);     // 22 => [38]
            map.insert(38, &[134]);     // 38 => [134]
            map.insert(87, &[2054]);     // 87 => [2054]
            map.insert(31, &[98]);     // 31 => [98]
            map.insert(50, &[162]);     // 50 => [162]
            map.insert(47, &[194]);     // 47 => [194]
            map.insert(94, &[2114]);     // 94 => [2114]
            map.insert(103, &[1154]);     // 103 => [1154]
            map.insert(115, &[2178]);     // 115 => [2178]
            map.insert(52, &[164]);     // 52 => [164]
            map.insert(117, &[2180]);     // 117 => [2180]
            map.insert(56, &[168]);     // 56 => [168]
            map.insert(104, &[392]);     // 104 => [392]
            map.insert(107, &[648]);     // 107 => [648]
            map.insert(109, &[1160]);     // 109 => [1160]
            map.insert(121, &[2184]);     // 121 => [2184]
            map.insert(59, &[176]);     // 59 => [176]
            map.insert(110, &[1168]);     // 110 => [1168]
            map.insert(122, &[2192]);     // 122 => [2192]
            map.insert(61, &[224]);     // 61 => [224]
            map.insert(124, &[2240]);     // 124 => [2240]
            map.insert(23, &[39]);     // 23 => [39]
            map.insert(39, &[135]);     // 39 => [135]
            map.insert(86, &[2055]);     // 86 => [2055]
            map.insert(30, &[99]);     // 30 => [99]
            map.insert(51, &[163]);     // 51 => [163]
            map.insert(46, &[195]);     // 46 => [195]
            map.insert(95, &[2115]);     // 95 => [2115]
            map.insert(102, &[1155]);     // 102 => [1155]
            map.insert(114, &[2179]);     // 114 => [2179]
            map.insert(53, &[165]);     // 53 => [165]
            map.insert(116, &[2181]);     // 116 => [2181]
            map.insert(57, &[169]);     // 57 => [169]
            map.insert(105, &[393]);     // 105 => [393]
            map.insert(106, &[649]);     // 106 => [649]
            map.insert(108, &[1161]);     // 108 => [1161]
            map.insert(120, &[2185]);     // 120 => [2185]
            map.insert(58, &[177]);     // 58 => [177]
            map.insert(111, &[1169]);     // 111 => [1169]
            map.insert(123, &[2193]);     // 123 => [2193]
            map.insert(60, &[225]);     // 60 => [225]
            map.insert(125, &[2241]);     // 125 => [2241]
            map.insert(54, &[166]);     // 54 => [166]
            map.insert(119, &[2182]);     // 119 => [2182]
            map.insert(63, &[226]);     // 63 => [226]
            map.insert(126, &[2242]);     // 126 => [2242]
            map.insert(55, &[167]);     // 55 => [167]
            map.insert(118, &[2183]);     // 118 => [2183]
            map.insert(62, &[227]);     // 62 => [227]
            map.insert(127, &[2243]);     // 127 => [2243]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode12_5 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode12_5 {
    fn name(&self) -> String {
        "[12, 5] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        12
    }

    fn dimension(&self) -> usize {
        5
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
        codeword.truncate(5);
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
        let code = GuavaCode12_5.generator_matrix();
        assert_eq!(code.ncols(), 12);
        assert_eq!(code.nrows(), 5);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode12_5;
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
            let code = GuavaCode12_5;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, true, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_5;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_5;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, false, false, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, true, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_5;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, false, false, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, false, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_5;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, true, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_5;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, true, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_5;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, true, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, false, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_5;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, true, true, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, true, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_5;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, false, false, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, false, true, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_5;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, true, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_5;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, true, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, false, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_5;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, false, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_5;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_5;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, true, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, true, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_5;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, false, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, false, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_5;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, true, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, false, true, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_5;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, true, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_5;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, true, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, false, true, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_5;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode12_5;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, true, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, true, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, true, false, false, true, false, false, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
