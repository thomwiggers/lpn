use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[13, 6]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode13_6;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 4417 ],
                &[ 4674 ],
                &[ 5188 ],
                &[ 6216 ],
                &[ 2704 ],
                &[ 3232 ],
                
            ], 13));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 7681 ],
                &[ 674 ],
                &[ 1060 ],
                &[ 2184 ],
                &[ 176 ],
                &[ 4160 ],
                &[ 7936 ],
                
            ], 13));
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
            map.insert(22, &[32]);     // 22 => [32]
            map.insert(32, &[64]);     // 32 => [64]
            map.insert(26, &[128]);     // 26 => [128]
            map.insert(64, &[256]);     // 64 => [256]
            map.insert(67, &[512]);     // 67 => [512]
            map.insert(69, &[1024]);     // 69 => [1024]
            map.insert(73, &[2048]);     // 73 => [2048]
            map.insert(97, &[4096]);     // 97 => [4096]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(23, &[33]);     // 23 => [33]
            map.insert(33, &[65]);     // 33 => [65]
            map.insert(27, &[129]);     // 27 => [129]
            map.insert(65, &[257]);     // 65 => [257]
            map.insert(66, &[513]);     // 66 => [513]
            map.insert(68, &[1025]);     // 68 => [1025]
            map.insert(72, &[2049]);     // 72 => [2049]
            map.insert(96, &[4097]);     // 96 => [4097]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(18, &[18]);     // 18 => [18]
            map.insert(20, &[34]);     // 20 => [34]
            map.insert(34, &[66]);     // 34 => [66]
            map.insert(24, &[130]);     // 24 => [130]
            map.insert(71, &[1026]);     // 71 => [1026]
            map.insert(75, &[2050]);     // 75 => [2050]
            map.insert(99, &[4098]);     // 99 => [4098]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(36, &[68]);     // 36 => [68]
            map.insert(30, &[132]);     // 30 => [132]
            map.insert(77, &[2052]);     // 77 => [2052]
            map.insert(101, &[4100]);     // 101 => [4100]
            map.insert(40, &[72]);     // 40 => [72]
            map.insert(105, &[4104]);     // 105 => [4104]
            map.insert(48, &[80]);     // 48 => [80]
            map.insert(80, &[272]);     // 80 => [272]
            map.insert(83, &[528]);     // 83 => [528]
            map.insert(85, &[1040]);     // 85 => [1040]
            map.insert(89, &[2064]);     // 89 => [2064]
            map.insert(113, &[4112]);     // 113 => [4112]
            map.insert(54, &[96]);     // 54 => [96]
            map.insert(86, &[288]);     // 86 => [288]
            map.insert(95, &[2080]);     // 95 => [2080]
            map.insert(119, &[4128]);     // 119 => [4128]
            map.insert(58, &[192]);     // 58 => [192]
            map.insert(90, &[384]);     // 90 => [384]
            map.insert(123, &[4224]);     // 123 => [4224]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(11, &[11]);     // 11 => [11]
            map.insert(19, &[19]);     // 19 => [19]
            map.insert(21, &[35]);     // 21 => [35]
            map.insert(35, &[67]);     // 35 => [67]
            map.insert(25, &[131]);     // 25 => [131]
            map.insert(70, &[1027]);     // 70 => [1027]
            map.insert(74, &[2051]);     // 74 => [2051]
            map.insert(98, &[4099]);     // 98 => [4099]
            map.insert(13, &[13]);     // 13 => [13]
            map.insert(37, &[69]);     // 37 => [69]
            map.insert(31, &[133]);     // 31 => [133]
            map.insert(76, &[2053]);     // 76 => [2053]
            map.insert(100, &[4101]);     // 100 => [4101]
            map.insert(41, &[73]);     // 41 => [73]
            map.insert(104, &[4105]);     // 104 => [4105]
            map.insert(49, &[81]);     // 49 => [81]
            map.insert(81, &[273]);     // 81 => [273]
            map.insert(82, &[529]);     // 82 => [529]
            map.insert(84, &[1041]);     // 84 => [1041]
            map.insert(88, &[2065]);     // 88 => [2065]
            map.insert(112, &[4113]);     // 112 => [4113]
            map.insert(55, &[97]);     // 55 => [97]
            map.insert(87, &[289]);     // 87 => [289]
            map.insert(94, &[2081]);     // 94 => [2081]
            map.insert(118, &[4129]);     // 118 => [4129]
            map.insert(59, &[193]);     // 59 => [193]
            map.insert(91, &[385]);     // 91 => [385]
            map.insert(122, &[4225]);     // 122 => [4225]
            map.insert(14, &[14]);     // 14 => [14]
            map.insert(38, &[70]);     // 38 => [70]
            map.insert(28, &[134]);     // 28 => [134]
            map.insert(79, &[2054]);     // 79 => [2054]
            map.insert(103, &[4102]);     // 103 => [4102]
            map.insert(42, &[74]);     // 42 => [74]
            map.insert(107, &[4106]);     // 107 => [4106]
            map.insert(50, &[82]);     // 50 => [82]
            map.insert(115, &[4114]);     // 115 => [4114]
            map.insert(52, &[98]);     // 52 => [98]
            map.insert(93, &[2082]);     // 93 => [2082]
            map.insert(117, &[4130]);     // 117 => [4130]
            map.insert(56, &[194]);     // 56 => [194]
            map.insert(121, &[4226]);     // 121 => [4226]
            map.insert(44, &[76]);     // 44 => [76]
            map.insert(109, &[4108]);     // 109 => [4108]
            map.insert(62, &[196]);     // 62 => [196]
            map.insert(127, &[4228]);     // 127 => [4228]
            map.insert(15, &[15]);     // 15 => [15]
            map.insert(39, &[71]);     // 39 => [71]
            map.insert(29, &[135]);     // 29 => [135]
            map.insert(78, &[2055]);     // 78 => [2055]
            map.insert(102, &[4103]);     // 102 => [4103]
            map.insert(43, &[75]);     // 43 => [75]
            map.insert(106, &[4107]);     // 106 => [4107]
            map.insert(51, &[83]);     // 51 => [83]
            map.insert(114, &[4115]);     // 114 => [4115]
            map.insert(53, &[99]);     // 53 => [99]
            map.insert(92, &[2083]);     // 92 => [2083]
            map.insert(116, &[4131]);     // 116 => [4131]
            map.insert(57, &[195]);     // 57 => [195]
            map.insert(120, &[4227]);     // 120 => [4227]
            map.insert(45, &[77]);     // 45 => [77]
            map.insert(108, &[4109]);     // 108 => [4109]
            map.insert(63, &[197]);     // 63 => [197]
            map.insert(126, &[4229]);     // 126 => [4229]
            map.insert(46, &[78]);     // 46 => [78]
            map.insert(111, &[4110]);     // 111 => [4110]
            map.insert(60, &[198]);     // 60 => [198]
            map.insert(125, &[4230]);     // 125 => [4230]
            map.insert(47, &[79]);     // 47 => [79]
            map.insert(110, &[4111]);     // 110 => [4111]
            map.insert(61, &[199]);     // 61 => [199]
            map.insert(124, &[4231]);     // 124 => [4231]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode13_6 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode13_6 {
    fn name(&self) -> String {
        "[13, 6] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        13
    }

    fn dimension(&self) -> usize {
        6
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
        let mut error = BinVector::with_capacity(13);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 13 / 64 + if 13 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(13) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(6);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[13 / 64] & !((1 << 13) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode13_6.generator_matrix();
        assert_eq!(code.ncols(), 13);
        assert_eq!(code.nrows(), 6);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode13_6;
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
            let code = GuavaCode13_6;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, false, true, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, true, false, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_6;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, false, true, true, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, false, false, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_6;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, true, true, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, true, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_6;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, false, true, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, false, false, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_6;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, true, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, true, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_6;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, false, false, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, false, false, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_6;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, false, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, true, false, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_6;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, false, false, false, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_6;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, false, true, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, false, true, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_6;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, true, true, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, true, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_6;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, false, false, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, false, true, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_6;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, true, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, false, false, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_6;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, false, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, false, false, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_6;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, false, true, false, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, false, true, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_6;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, false, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, false, false, true, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_6;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, false, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, false, false, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_6;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, false, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, false, false, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_6;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, false, false, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, false, false, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_6;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, false, true, true, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, true, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode13_6;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, false, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, false, false, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, true, false, true, false, false, false, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
