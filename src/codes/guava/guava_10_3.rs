use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[10, 3]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode10_3;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 681 ],
                &[ 370 ],
                &[ 996 ],
                
            ], 10));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 289 ],
                &[ 546 ],
                &[ 804 ],
                &[ 296 ],
                &[ 560 ],
                &[ 320 ],
                &[ 640 ],
                
            ], 10));
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
            map.insert(31, &[32]);     // 31 => [32]
            map.insert(32, &[64]);     // 32 => [64]
            map.insert(64, &[128]);     // 64 => [128]
            map.insert(45, &[256]);     // 45 => [256]
            map.insert(86, &[512]);     // 86 => [512]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(30, &[33]);     // 30 => [33]
            map.insert(33, &[65]);     // 33 => [65]
            map.insert(65, &[129]);     // 65 => [129]
            map.insert(44, &[257]);     // 44 => [257]
            map.insert(87, &[513]);     // 87 => [513]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(18, &[18]);     // 18 => [18]
            map.insert(29, &[34]);     // 29 => [34]
            map.insert(34, &[66]);     // 34 => [66]
            map.insert(66, &[130]);     // 66 => [130]
            map.insert(47, &[258]);     // 47 => [258]
            map.insert(84, &[514]);     // 84 => [514]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(20, &[20]);     // 20 => [20]
            map.insert(27, &[36]);     // 27 => [36]
            map.insert(36, &[68]);     // 36 => [68]
            map.insert(68, &[132]);     // 68 => [132]
            map.insert(41, &[260]);     // 41 => [260]
            map.insert(82, &[516]);     // 82 => [516]
            map.insert(24, &[24]);     // 24 => [24]
            map.insert(23, &[40]);     // 23 => [40]
            map.insert(40, &[72]);     // 40 => [72]
            map.insert(72, &[136]);     // 72 => [136]
            map.insert(37, &[264]);     // 37 => [264]
            map.insert(94, &[520]);     // 94 => [520]
            map.insert(15, &[48]);     // 15 => [48]
            map.insert(48, &[80]);     // 48 => [80]
            map.insert(80, &[144]);     // 80 => [144]
            map.insert(61, &[272]);     // 61 => [272]
            map.insert(70, &[528]);     // 70 => [528]
            map.insert(63, &[96]);     // 63 => [96]
            map.insert(95, &[160]);     // 95 => [160]
            map.insert(50, &[288]);     // 50 => [288]
            map.insert(73, &[544]);     // 73 => [544]
            map.insert(96, &[192]);     // 96 => [192]
            map.insert(13, &[320]);     // 13 => [320]
            map.insert(118, &[576]);     // 118 => [576]
            map.insert(109, &[384]);     // 109 => [384]
            map.insert(22, &[640]);     // 22 => [640]
            map.insert(123, &[768]);     // 123 => [768]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(11, &[11]);     // 11 => [11]
            map.insert(19, &[19]);     // 19 => [19]
            map.insert(28, &[35]);     // 28 => [35]
            map.insert(35, &[67]);     // 35 => [67]
            map.insert(67, &[131]);     // 67 => [131]
            map.insert(46, &[259]);     // 46 => [259]
            map.insert(85, &[515]);     // 85 => [515]
            map.insert(21, &[21]);     // 21 => [21]
            map.insert(26, &[37]);     // 26 => [37]
            map.insert(69, &[133]);     // 69 => [133]
            map.insert(83, &[517]);     // 83 => [517]
            map.insert(25, &[25]);     // 25 => [25]
            map.insert(14, &[49]);     // 14 => [49]
            map.insert(49, &[81]);     // 49 => [81]
            map.insert(81, &[145]);     // 81 => [145]
            map.insert(60, &[273]);     // 60 => [273]
            map.insert(71, &[529]);     // 71 => [529]
            map.insert(62, &[97]);     // 62 => [97]
            map.insert(51, &[289]);     // 51 => [289]
            map.insert(97, &[193]);     // 97 => [193]
            map.insert(119, &[577]);     // 119 => [577]
            map.insert(108, &[385]);     // 108 => [385]
            map.insert(122, &[769]);     // 122 => [769]
            map.insert(38, &[70]);     // 38 => [70]
            map.insert(43, &[262]);     // 43 => [262]
            map.insert(42, &[74]);     // 42 => [74]
            map.insert(74, &[138]);     // 74 => [138]
            map.insert(39, &[266]);     // 39 => [266]
            map.insert(92, &[522]);     // 92 => [522]
            map.insert(93, &[162]);     // 93 => [162]
            map.insert(75, &[546]);     // 75 => [546]
            map.insert(98, &[194]);     // 98 => [194]
            map.insert(116, &[578]);     // 116 => [578]
            map.insert(111, &[386]);     // 111 => [386]
            map.insert(121, &[770]);     // 121 => [770]
            map.insert(76, &[140]);     // 76 => [140]
            map.insert(90, &[524]);     // 90 => [524]
            map.insert(52, &[84]);     // 52 => [84]
            map.insert(57, &[276]);     // 57 => [276]
            map.insert(59, &[100]);     // 59 => [100]
            map.insert(91, &[164]);     // 91 => [164]
            map.insert(54, &[292]);     // 54 => [292]
            map.insert(77, &[548]);     // 77 => [548]
            map.insert(100, &[196]);     // 100 => [196]
            map.insert(114, &[580]);     // 114 => [580]
            map.insert(105, &[388]);     // 105 => [388]
            map.insert(127, &[772]);     // 127 => [772]
            map.insert(56, &[88]);     // 56 => [88]
            map.insert(88, &[152]);     // 88 => [152]
            map.insert(53, &[280]);     // 53 => [280]
            map.insert(78, &[536]);     // 78 => [536]
            map.insert(55, &[104]);     // 55 => [104]
            map.insert(58, &[296]);     // 58 => [296]
            map.insert(104, &[200]);     // 104 => [200]
            map.insert(126, &[584]);     // 126 => [584]
            map.insert(101, &[392]);     // 101 => [392]
            map.insert(115, &[776]);     // 115 => [776]
            map.insert(79, &[176]);     // 79 => [176]
            map.insert(89, &[560]);     // 89 => [560]
            map.insert(112, &[208]);     // 112 => [208]
            map.insert(102, &[592]);     // 102 => [592]
            map.insert(125, &[400]);     // 125 => [400]
            map.insert(107, &[784]);     // 107 => [784]
            map.insert(99, &[195]);     // 99 => [195]
            map.insert(117, &[579]);     // 117 => [579]
            map.insert(110, &[387]);     // 110 => [387]
            map.insert(120, &[771]);     // 120 => [771]
            map.insert(113, &[209]);     // 113 => [209]
            map.insert(103, &[593]);     // 103 => [593]
            map.insert(124, &[401]);     // 124 => [401]
            map.insert(106, &[785]);     // 106 => [785]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode10_3 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode10_3 {
    fn name(&self) -> String {
        "[10, 3] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        10
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
        let mut error = BinVector::with_capacity(10);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 10 / 64 + if 10 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(10) };
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
        
        debug_assert_eq!(c[10 / 64] & !((1 << 10) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode10_3.generator_matrix();
        assert_eq!(code.ncols(), 10);
        assert_eq!(code.nrows(), 3);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode10_3;
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
            let code = GuavaCode10_3;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_3;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_3;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_3;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_3;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_3;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_3;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_3;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_3;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_3;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_3;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_3;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_3;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_3;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_3;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_3;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_3;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_3;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_3;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode10_3;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, true, false, true, false, true, false, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
