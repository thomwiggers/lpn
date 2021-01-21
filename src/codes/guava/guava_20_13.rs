use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[20, 13]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode20_13;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 598017 ],
                &[ 663554 ],
                &[ 794628 ],
                &[ 114696 ],
                &[ 180240 ],
                &[ 311328 ],
                &[ 573504 ],
                &[ 606336 ],
                &[ 672000 ],
                &[ 803328 ],
                &[ 623616 ],
                &[ 690176 ],
                &[ 823296 ],
                
            ], 20));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 408369 ],
                &[ 133394 ],
                &[ 266788 ],
                &[ 516152 ],
                &[ 581696 ],
                &[ 1000320 ],
                &[ 1022976 ],
                
            ], 20));
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
            map.insert(13, &[32]);     // 13 => [32]
            map.insert(16, &[64]);     // 16 => [64]
            map.insert(32, &[128]);     // 32 => [128]
            map.insert(35, &[256]);     // 35 => [256]
            map.insert(37, &[512]);     // 37 => [512]
            map.insert(64, &[1024]);     // 64 => [1024]
            map.insert(67, &[2048]);     // 67 => [2048]
            map.insert(69, &[4096]);     // 69 => [4096]
            map.insert(25, &[8192]);     // 25 => [8192]
            map.insert(56, &[16384]);     // 56 => [16384]
            map.insert(88, &[32768]);     // 88 => [32768]
            map.insert(104, &[65536]);     // 104 => [65536]
            map.insert(107, &[131072]);     // 107 => [131072]
            map.insert(109, &[262144]);     // 109 => [262144]
            map.insert(112, &[524288]);     // 112 => [524288]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(10, &[17]);     // 10 => [17]
            map.insert(12, &[33]);     // 12 => [33]
            map.insert(17, &[65]);     // 17 => [65]
            map.insert(33, &[129]);     // 33 => [129]
            map.insert(34, &[257]);     // 34 => [257]
            map.insert(36, &[513]);     // 36 => [513]
            map.insert(65, &[1025]);     // 65 => [1025]
            map.insert(66, &[2049]);     // 66 => [2049]
            map.insert(68, &[4097]);     // 68 => [4097]
            map.insert(24, &[8193]);     // 24 => [8193]
            map.insert(57, &[16385]);     // 57 => [16385]
            map.insert(89, &[32769]);     // 89 => [32769]
            map.insert(105, &[65537]);     // 105 => [65537]
            map.insert(106, &[131073]);     // 106 => [131073]
            map.insert(108, &[262145]);     // 108 => [262145]
            map.insert(113, &[524289]);     // 113 => [524289]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(15, &[34]);     // 15 => [34]
            map.insert(18, &[66]);     // 18 => [66]
            map.insert(39, &[514]);     // 39 => [514]
            map.insert(71, &[4098]);     // 71 => [4098]
            map.insert(27, &[8194]);     // 27 => [8194]
            map.insert(58, &[16386]);     // 58 => [16386]
            map.insert(90, &[32770]);     // 90 => [32770]
            map.insert(111, &[262146]);     // 111 => [262146]
            map.insert(114, &[524290]);     // 114 => [524290]
            map.insert(20, &[68]);     // 20 => [68]
            map.insert(29, &[8196]);     // 29 => [8196]
            map.insert(60, &[16388]);     // 60 => [16388]
            map.insert(92, &[32772]);     // 92 => [32772]
            map.insert(116, &[524292]);     // 116 => [524292]
            map.insert(40, &[136]);     // 40 => [136]
            map.insert(43, &[264]);     // 43 => [264]
            map.insert(45, &[520]);     // 45 => [520]
            map.insert(72, &[1032]);     // 72 => [1032]
            map.insert(75, &[2056]);     // 75 => [2056]
            map.insert(77, &[4104]);     // 77 => [4104]
            map.insert(48, &[16392]);     // 48 => [16392]
            map.insert(80, &[32776]);     // 80 => [32776]
            map.insert(96, &[65544]);     // 96 => [65544]
            map.insert(99, &[131080]);     // 99 => [131080]
            map.insert(101, &[262152]);     // 101 => [262152]
            map.insert(120, &[524296]);     // 120 => [524296]
            map.insert(46, &[528]);     // 46 => [528]
            map.insert(78, &[4112]);     // 78 => [4112]
            map.insert(51, &[16400]);     // 51 => [16400]
            map.insert(83, &[32784]);     // 83 => [32784]
            map.insert(102, &[262160]);     // 102 => [262160]
            map.insert(123, &[524304]);     // 123 => [524304]
            map.insert(53, &[16416]);     // 53 => [16416]
            map.insert(85, &[32800]);     // 85 => [32800]
            map.insert(125, &[524320]);     // 125 => [524320]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(14, &[35]);     // 14 => [35]
            map.insert(19, &[67]);     // 19 => [67]
            map.insert(38, &[515]);     // 38 => [515]
            map.insert(70, &[4099]);     // 70 => [4099]
            map.insert(26, &[8195]);     // 26 => [8195]
            map.insert(59, &[16387]);     // 59 => [16387]
            map.insert(91, &[32771]);     // 91 => [32771]
            map.insert(110, &[262147]);     // 110 => [262147]
            map.insert(115, &[524291]);     // 115 => [524291]
            map.insert(21, &[69]);     // 21 => [69]
            map.insert(28, &[8197]);     // 28 => [8197]
            map.insert(61, &[16389]);     // 61 => [16389]
            map.insert(93, &[32773]);     // 93 => [32773]
            map.insert(117, &[524293]);     // 117 => [524293]
            map.insert(41, &[137]);     // 41 => [137]
            map.insert(42, &[265]);     // 42 => [265]
            map.insert(44, &[521]);     // 44 => [521]
            map.insert(73, &[1033]);     // 73 => [1033]
            map.insert(74, &[2057]);     // 74 => [2057]
            map.insert(76, &[4105]);     // 76 => [4105]
            map.insert(49, &[16393]);     // 49 => [16393]
            map.insert(81, &[32777]);     // 81 => [32777]
            map.insert(97, &[65545]);     // 97 => [65545]
            map.insert(98, &[131081]);     // 98 => [131081]
            map.insert(100, &[262153]);     // 100 => [262153]
            map.insert(121, &[524297]);     // 121 => [524297]
            map.insert(47, &[529]);     // 47 => [529]
            map.insert(79, &[4113]);     // 79 => [4113]
            map.insert(50, &[16401]);     // 50 => [16401]
            map.insert(82, &[32785]);     // 82 => [32785]
            map.insert(103, &[262161]);     // 103 => [262161]
            map.insert(122, &[524305]);     // 122 => [524305]
            map.insert(52, &[16417]);     // 52 => [16417]
            map.insert(84, &[32801]);     // 84 => [32801]
            map.insert(124, &[524321]);     // 124 => [524321]
            map.insert(22, &[70]);     // 22 => [70]
            map.insert(31, &[8198]);     // 31 => [8198]
            map.insert(62, &[16390]);     // 62 => [16390]
            map.insert(94, &[32774]);     // 94 => [32774]
            map.insert(118, &[524294]);     // 118 => [524294]
            map.insert(55, &[16418]);     // 55 => [16418]
            map.insert(87, &[32802]);     // 87 => [32802]
            map.insert(127, &[524322]);     // 127 => [524322]
            map.insert(23, &[71]);     // 23 => [71]
            map.insert(30, &[8199]);     // 30 => [8199]
            map.insert(63, &[16391]);     // 63 => [16391]
            map.insert(95, &[32775]);     // 95 => [32775]
            map.insert(119, &[524295]);     // 119 => [524295]
            map.insert(54, &[16419]);     // 54 => [16419]
            map.insert(86, &[32803]);     // 86 => [32803]
            map.insert(126, &[524323]);     // 126 => [524323]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode20_13 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode20_13 {
    fn name(&self) -> String {
        "[20, 13] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        20
    }

    fn dimension(&self) -> usize {
        13
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
        let mut error = BinVector::with_capacity(20);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 20 / 64 + if 20 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(20) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(13);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[20 / 64] & !((1 << 20) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode20_13.generator_matrix();
        assert_eq!(code.ncols(), 20);
        assert_eq!(code.nrows(), 13);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode20_13;
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
            let code = GuavaCode20_13;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, false, false, true, false, false, true, true, false, true, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, false, true, false, false, true, true, false, true, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_13;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, true, false, false, false, true, true, true, false, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, true, false, false, false, true, true, true, false, true, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_13;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, false, false, false, true, false, true, false, false, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, false, false, false, false, true, false, true, false, false, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_13;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, true, true, false, false, true, false, true, true, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, true, true, true, false, false, true, false, true, true, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_13;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, true, false, true, false, true, true, false, true, true, true, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, false, true, false, true, true, false, true, false, true, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_13;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, true, false, true, true, false, true, true, true, true, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, false, false, false, true, true, false, true, true, true, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_13;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, false, true, false, true, false, false, true, false, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, false, true, false, true, false, false, true, false, false, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_13;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, true, false, false, false, false, true, true, true, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, false, true, false, false, false, false, true, true, true, true, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_13;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, true, true, true, false, false, false, true, false, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, true, true, true, false, false, false, true, false, false, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_13;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, false, true, false, false, true, false, true, false, false, true, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, false, true, false, false, true, true, true, false, false, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_13;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, true, true, false, true, true, true, false, false, false, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, true, true, true, false, true, true, true, false, false, false, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_13;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, false, false, true, false, true, true, true, false, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, false, false, false, true, true, true, true, true, false, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_13;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, false, false, true, true, false, false, false, false, true, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, false, false, true, true, false, false, false, false, true, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_13;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, false, true, false, true, true, true, true, false, true, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, false, false, true, false, false, true, true, true, false, true, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_13;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, true, false, true, false, true, false, true, false, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, true, false, true, false, true, false, true, false, false, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_13;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, false, false, true, true, true, true, true, true, false, false, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, false, true, true, true, true, true, true, true, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_13;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, false, false, true, true, true, true, false, true, false, false, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, false, false, true, true, true, true, false, true, false, false, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_13;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, false, true, false, true, true, false, false, true, false, true, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, false, true, false, true, true, false, false, true, false, false, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_13;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, true, true, true, true, true, false, true, true, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, true, true, true, true, true, false, true, false, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode20_13;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, true, true, false, false, true, false, true, false, false, false, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, true, false, false, true, false, true, true, false, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, false, false, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
