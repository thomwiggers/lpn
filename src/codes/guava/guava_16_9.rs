use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[16, 9]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode16_9;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 54785 ],
                &[ 58882 ],
                &[ 34308 ],
                &[ 35336 ],
                &[ 37392 ],
                &[ 41504 ],
                &[ 49728 ],
                &[ 21632 ],
                &[ 25856 ],
                
            ], 16));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 40289 ],
                &[ 8482 ],
                &[ 17476 ],
                &[ 2056 ],
                &[ 28784 ],
                &[ 64896 ],
                &[ 33280 ],
                
            ], 16));
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
            map.insert(19, &[32]);     // 19 => [32]
            map.insert(21, &[64]);     // 21 => [64]
            map.insert(32, &[128]);     // 32 => [128]
            map.insert(35, &[256]);     // 35 => [256]
            map.insert(64, &[512]);     // 64 => [512]
            map.insert(37, &[1024]);     // 37 => [1024]
            map.insert(41, &[2048]);     // 41 => [2048]
            map.insert(49, &[4096]);     // 49 => [4096]
            map.insert(50, &[8192]);     // 50 => [8192]
            map.insert(52, &[16384]);     // 52 => [16384]
            map.insert(97, &[32768]);     // 97 => [32768]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(18, &[33]);     // 18 => [33]
            map.insert(20, &[65]);     // 20 => [65]
            map.insert(33, &[129]);     // 33 => [129]
            map.insert(34, &[257]);     // 34 => [257]
            map.insert(65, &[513]);     // 65 => [513]
            map.insert(36, &[1025]);     // 36 => [1025]
            map.insert(40, &[2049]);     // 40 => [2049]
            map.insert(48, &[4097]);     // 48 => [4097]
            map.insert(51, &[8193]);     // 51 => [8193]
            map.insert(53, &[16385]);     // 53 => [16385]
            map.insert(96, &[32769]);     // 96 => [32769]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(23, &[66]);     // 23 => [66]
            map.insert(66, &[514]);     // 66 => [514]
            map.insert(39, &[1026]);     // 39 => [1026]
            map.insert(43, &[2050]);     // 43 => [2050]
            map.insert(54, &[16386]);     // 54 => [16386]
            map.insert(99, &[32770]);     // 99 => [32770]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(68, &[516]);     // 68 => [516]
            map.insert(45, &[2052]);     // 45 => [2052]
            map.insert(101, &[32772]);     // 101 => [32772]
            map.insert(24, &[24]);     // 24 => [24]
            map.insert(27, &[40]);     // 27 => [40]
            map.insert(29, &[72]);     // 29 => [72]
            map.insert(72, &[520]);     // 72 => [520]
            map.insert(57, &[4104]);     // 57 => [4104]
            map.insert(58, &[8200]);     // 58 => [8200]
            map.insert(60, &[16392]);     // 60 => [16392]
            map.insert(105, &[32776]);     // 105 => [32776]
            map.insert(80, &[528]);     // 80 => [528]
            map.insert(113, &[32784]);     // 113 => [32784]
            map.insert(83, &[544]);     // 83 => [544]
            map.insert(114, &[32800]);     // 114 => [32800]
            map.insert(85, &[576]);     // 85 => [576]
            map.insert(116, &[32832]);     // 116 => [32832]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(11, &[11]);     // 11 => [11]
            map.insert(22, &[67]);     // 22 => [67]
            map.insert(67, &[515]);     // 67 => [515]
            map.insert(38, &[1027]);     // 38 => [1027]
            map.insert(42, &[2051]);     // 42 => [2051]
            map.insert(55, &[16387]);     // 55 => [16387]
            map.insert(98, &[32771]);     // 98 => [32771]
            map.insert(13, &[13]);     // 13 => [13]
            map.insert(69, &[517]);     // 69 => [517]
            map.insert(44, &[2053]);     // 44 => [2053]
            map.insert(100, &[32773]);     // 100 => [32773]
            map.insert(25, &[25]);     // 25 => [25]
            map.insert(26, &[41]);     // 26 => [41]
            map.insert(28, &[73]);     // 28 => [73]
            map.insert(73, &[521]);     // 73 => [521]
            map.insert(56, &[4105]);     // 56 => [4105]
            map.insert(59, &[8201]);     // 59 => [8201]
            map.insert(61, &[16393]);     // 61 => [16393]
            map.insert(104, &[32777]);     // 104 => [32777]
            map.insert(81, &[529]);     // 81 => [529]
            map.insert(112, &[32785]);     // 112 => [32785]
            map.insert(82, &[545]);     // 82 => [545]
            map.insert(115, &[32801]);     // 115 => [32801]
            map.insert(84, &[577]);     // 84 => [577]
            map.insert(117, &[32833]);     // 117 => [32833]
            map.insert(14, &[14]);     // 14 => [14]
            map.insert(70, &[518]);     // 70 => [518]
            map.insert(47, &[2054]);     // 47 => [2054]
            map.insert(103, &[32774]);     // 103 => [32774]
            map.insert(31, &[74]);     // 31 => [74]
            map.insert(74, &[522]);     // 74 => [522]
            map.insert(62, &[16394]);     // 62 => [16394]
            map.insert(107, &[32778]);     // 107 => [32778]
            map.insert(87, &[578]);     // 87 => [578]
            map.insert(118, &[32834]);     // 118 => [32834]
            map.insert(76, &[524]);     // 76 => [524]
            map.insert(109, &[32780]);     // 109 => [32780]
            map.insert(88, &[536]);     // 88 => [536]
            map.insert(121, &[32792]);     // 121 => [32792]
            map.insert(91, &[552]);     // 91 => [552]
            map.insert(122, &[32808]);     // 122 => [32808]
            map.insert(93, &[584]);     // 93 => [584]
            map.insert(124, &[32840]);     // 124 => [32840]
            map.insert(15, &[15]);     // 15 => [15]
            map.insert(71, &[519]);     // 71 => [519]
            map.insert(46, &[2055]);     // 46 => [2055]
            map.insert(102, &[32775]);     // 102 => [32775]
            map.insert(30, &[75]);     // 30 => [75]
            map.insert(75, &[523]);     // 75 => [523]
            map.insert(63, &[16395]);     // 63 => [16395]
            map.insert(106, &[32779]);     // 106 => [32779]
            map.insert(86, &[579]);     // 86 => [579]
            map.insert(119, &[32835]);     // 119 => [32835]
            map.insert(77, &[525]);     // 77 => [525]
            map.insert(108, &[32781]);     // 108 => [32781]
            map.insert(89, &[537]);     // 89 => [537]
            map.insert(120, &[32793]);     // 120 => [32793]
            map.insert(90, &[553]);     // 90 => [553]
            map.insert(123, &[32809]);     // 123 => [32809]
            map.insert(92, &[585]);     // 92 => [585]
            map.insert(125, &[32841]);     // 125 => [32841]
            map.insert(78, &[526]);     // 78 => [526]
            map.insert(111, &[32782]);     // 111 => [32782]
            map.insert(95, &[586]);     // 95 => [586]
            map.insert(126, &[32842]);     // 126 => [32842]
            map.insert(79, &[527]);     // 79 => [527]
            map.insert(110, &[32783]);     // 110 => [32783]
            map.insert(94, &[587]);     // 94 => [587]
            map.insert(127, &[32843]);     // 127 => [32843]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode16_9 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode16_9 {
    fn name(&self) -> String {
        "[16, 9] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        16
    }

    fn dimension(&self) -> usize {
        9
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
        codeword.truncate(9);
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
        let code = GuavaCode16_9.generator_matrix();
        assert_eq!(code.ncols(), 16);
        assert_eq!(code.nrows(), 9);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode16_9;
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
            let code = GuavaCode16_9;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, true, true, false, false, false, false, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, true, true, false, false, true, false, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_9;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, false, true, true, true, true, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, true, false, true, true, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_9;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, false, true, true, false, true, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, false, true, true, false, true, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_9;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, true, false, false, false, true, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, false, false, true, true, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_9;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, false, true, true, false, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, false, false, true, true, false, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_9;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, false, true, true, true, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, false, true, true, true, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_9;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, false, false, true, true, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, false, false, true, true, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_9;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, false, false, true, true, false, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, false, false, false, true, true, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_9;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, false, true, false, true, true, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, false, false, true, false, true, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_9;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, true, false, false, false, false, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, false, false, false, false, true, false, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_9;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, true, true, false, true, false, false, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, true, true, false, true, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_9;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, false, false, true, true, true, false, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, false, false, true, true, true, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_9;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, false, false, false, false, true, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, false, false, false, true, false, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_9;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, true, false, false, false, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, true, false, false, true, true, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_9;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, true, false, true, true, false, true, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, false, false, true, false, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_9;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, false, true, true, false, false, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, false, true, true, false, false, false, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_9;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, false, true, true, true, true, true, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, true, true, true, true, false, true, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_9;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, true, false, false, false, true, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, true, true, false, true, false, true, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_9;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, true, false, true, true, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, false, false, true, true, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode16_9;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, true, false, true, true, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, true, false, false, true, true, false, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, true, true, false, true, false, true, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
