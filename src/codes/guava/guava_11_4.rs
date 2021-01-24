use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[11, 4]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode11_4;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 433 ],
                &[ 1362 ],
                &[ 740 ],
                &[ 1992 ],
                
            ], 11));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 1281 ],
                &[ 578 ],
                &[ 1092 ],
                &[ 1608 ],
                &[ 1872 ],
                &[ 352 ],
                &[ 1920 ],
                
            ], 11));
            let matrix_t = Box::new(matrix.transposed());
            PARITY_MATRIX = Box::into_raw(matrix);
            PARITY_MATRIX_T = Box::into_raw(matrix_t);

            let mut map = Box::new(FnvHashMap::with_capacity_and_hasher(128, Default::default()));
            map.insert(0, &[0]);     // 0 => [0]
            map.insert(1, &[1]);     // 1 => [1]
            map.insert(2, &[2]);     // 2 => [2]
            map.insert(4, &[4]);     // 4 => [4]
            map.insert(8, &[8]);     // 8 => [8]
            map.insert(16, &[16]);     // 16 => [16]
            map.insert(32, &[32]);     // 32 => [32]
            map.insert(62, &[64]);     // 62 => [64]
            map.insert(64, &[128]);     // 64 => [128]
            map.insert(113, &[256]);     // 113 => [256]
            map.insert(90, &[512]);     // 90 => [512]
            map.insert(93, &[1024]);     // 93 => [1024]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(33, &[33]);     // 33 => [33]
            map.insert(63, &[65]);     // 63 => [65]
            map.insert(65, &[129]);     // 65 => [129]
            map.insert(112, &[257]);     // 112 => [257]
            map.insert(91, &[513]);     // 91 => [513]
            map.insert(92, &[1025]);     // 92 => [1025]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(18, &[18]);     // 18 => [18]
            map.insert(34, &[34]);     // 34 => [34]
            map.insert(60, &[66]);     // 60 => [66]
            map.insert(66, &[130]);     // 66 => [130]
            map.insert(115, &[258]);     // 115 => [258]
            map.insert(88, &[514]);     // 88 => [514]
            map.insert(95, &[1026]);     // 95 => [1026]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(20, &[20]);     // 20 => [20]
            map.insert(36, &[36]);     // 36 => [36]
            map.insert(58, &[68]);     // 58 => [68]
            map.insert(68, &[132]);     // 68 => [132]
            map.insert(117, &[260]);     // 117 => [260]
            map.insert(94, &[516]);     // 94 => [516]
            map.insert(89, &[1028]);     // 89 => [1028]
            map.insert(24, &[24]);     // 24 => [24]
            map.insert(40, &[40]);     // 40 => [40]
            map.insert(54, &[72]);     // 54 => [72]
            map.insert(72, &[136]);     // 72 => [136]
            map.insert(121, &[264]);     // 121 => [264]
            map.insert(82, &[520]);     // 82 => [520]
            map.insert(85, &[1032]);     // 85 => [1032]
            map.insert(48, &[48]);     // 48 => [48]
            map.insert(46, &[80]);     // 46 => [80]
            map.insert(80, &[144]);     // 80 => [144]
            map.insert(97, &[272]);     // 97 => [272]
            map.insert(74, &[528]);     // 74 => [528]
            map.insert(77, &[1040]);     // 77 => [1040]
            map.insert(30, &[96]);     // 30 => [96]
            map.insert(96, &[160]);     // 96 => [160]
            map.insert(81, &[288]);     // 81 => [288]
            map.insert(122, &[544]);     // 122 => [544]
            map.insert(125, &[1056]);     // 125 => [1056]
            map.insert(126, &[192]);     // 126 => [192]
            map.insert(79, &[320]);     // 79 => [320]
            map.insert(100, &[576]);     // 100 => [576]
            map.insert(99, &[1088]);     // 99 => [1088]
            map.insert(49, &[384]);     // 49 => [384]
            map.insert(26, &[640]);     // 26 => [640]
            map.insert(29, &[1152]);     // 29 => [1152]
            map.insert(43, &[768]);     // 43 => [768]
            map.insert(44, &[1280]);     // 44 => [1280]
            map.insert(7, &[1536]);     // 7 => [1536]
            map.insert(11, &[11]);     // 11 => [11]
            map.insert(19, &[19]);     // 19 => [19]
            map.insert(35, &[35]);     // 35 => [35]
            map.insert(61, &[67]);     // 61 => [67]
            map.insert(67, &[131]);     // 67 => [131]
            map.insert(114, &[259]);     // 114 => [259]
            map.insert(13, &[13]);     // 13 => [13]
            map.insert(21, &[21]);     // 21 => [21]
            map.insert(37, &[37]);     // 37 => [37]
            map.insert(59, &[69]);     // 59 => [69]
            map.insert(69, &[133]);     // 69 => [133]
            map.insert(116, &[261]);     // 116 => [261]
            map.insert(25, &[25]);     // 25 => [25]
            map.insert(41, &[41]);     // 41 => [41]
            map.insert(55, &[73]);     // 55 => [73]
            map.insert(73, &[137]);     // 73 => [137]
            map.insert(120, &[265]);     // 120 => [265]
            map.insert(83, &[521]);     // 83 => [521]
            map.insert(84, &[1033]);     // 84 => [1033]
            map.insert(47, &[81]);     // 47 => [81]
            map.insert(75, &[529]);     // 75 => [529]
            map.insert(76, &[1041]);     // 76 => [1041]
            map.insert(31, &[97]);     // 31 => [97]
            map.insert(123, &[545]);     // 123 => [545]
            map.insert(124, &[1057]);     // 124 => [1057]
            map.insert(127, &[193]);     // 127 => [193]
            map.insert(78, &[321]);     // 78 => [321]
            map.insert(101, &[577]);     // 101 => [577]
            map.insert(98, &[1089]);     // 98 => [1089]
            map.insert(27, &[641]);     // 27 => [641]
            map.insert(28, &[1153]);     // 28 => [1153]
            map.insert(42, &[769]);     // 42 => [769]
            map.insert(45, &[1281]);     // 45 => [1281]
            map.insert(14, &[14]);     // 14 => [14]
            map.insert(22, &[22]);     // 22 => [22]
            map.insert(38, &[38]);     // 38 => [38]
            map.insert(56, &[70]);     // 56 => [70]
            map.insert(70, &[134]);     // 70 => [134]
            map.insert(119, &[262]);     // 119 => [262]
            map.insert(52, &[74]);     // 52 => [74]
            map.insert(87, &[1034]);     // 87 => [1034]
            map.insert(50, &[50]);     // 50 => [50]
            map.insert(102, &[578]);     // 102 => [578]
            map.insert(51, &[386]);     // 51 => [386]
            map.insert(86, &[524]);     // 86 => [524]
            map.insert(103, &[1092]);     // 103 => [1092]
            map.insert(53, &[388]);     // 53 => [388]
            map.insert(105, &[280]);     // 105 => [280]
            map.insert(104, &[168]);     // 104 => [168]
            map.insert(118, &[200]);     // 118 => [200]
            map.insert(71, &[328]);     // 71 => [328]
            map.insert(108, &[584]);     // 108 => [584]
            map.insert(107, &[1096]);     // 107 => [1096]
            map.insert(57, &[392]);     // 57 => [392]
            map.insert(15, &[1544]);     // 15 => [1544]
            map.insert(106, &[560]);     // 106 => [560]
            map.insert(109, &[1072]);     // 109 => [1072]
            map.insert(110, &[208]);     // 110 => [208]
            map.insert(23, &[1552]);     // 23 => [1552]
            map.insert(111, &[352]);     // 111 => [352]
            map.insert(39, &[1568]);     // 39 => [1568]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode11_4 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode11_4 {
    fn name(&self) -> String {
        "[11, 4] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        11
    }

    fn dimension(&self) -> usize {
        4
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
        let mut error = BinVector::with_capacity(11);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 11 / 64 + if 11 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(11) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(4);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[11 / 64] & !((1 << 11) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode11_4.generator_matrix();
        assert_eq!(code.ncols(), 11);
        assert_eq!(code.nrows(), 4);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode11_4;
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
            let code = GuavaCode11_4;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_4;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_4;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_4;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_4;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, true, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_4;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_4;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_4;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_4;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_4;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_4;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, false, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_4;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_4;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_4;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, false, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_4;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_4;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_4;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, true, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_4;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, true, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_4;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode11_4;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, true, true, false, true, true, false, false ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
