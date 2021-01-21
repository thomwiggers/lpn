use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[21, 14]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode21_14;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 1130497 ],
                &[ 1196034 ],
                &[ 1327108 ],
                &[ 1589256 ],
                &[ 229392 ],
                &[ 360480 ],
                &[ 622656 ],
                &[ 1147008 ],
                &[ 1212672 ],
                &[ 1344000 ],
                &[ 1606656 ],
                &[ 1247232 ],
                &[ 1380352 ],
                &[ 1646592 ],
                
            ], 21));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 2000641 ],
                &[ 1241442 ],
                &[ 266788 ],
                &[ 533576 ],
                &[ 1032304 ],
                &[ 1001344 ],
                &[ 2045952 ],
                
            ], 21));
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
            map.insert(26, &[64]);     // 26 => [64]
            map.insert(32, &[128]);     // 32 => [128]
            map.insert(35, &[256]);     // 35 => [256]
            map.insert(37, &[512]);     // 37 => [512]
            map.insert(41, &[1024]);     // 41 => [1024]
            map.insert(64, &[2048]);     // 64 => [2048]
            map.insert(70, &[4096]);     // 70 => [4096]
            map.insert(74, &[8192]);     // 74 => [8192]
            map.insert(50, &[16384]);     // 50 => [16384]
            map.insert(19, &[32768]);     // 19 => [32768]
            map.insert(112, &[65536]);     // 112 => [65536]
            map.insert(115, &[131072]);     // 115 => [131072]
            map.insert(117, &[262144]);     // 117 => [262144]
            map.insert(121, &[524288]);     // 121 => [524288]
            map.insert(67, &[1048576]);     // 67 => [1048576]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(23, &[33]);     // 23 => [33]
            map.insert(27, &[65]);     // 27 => [65]
            map.insert(33, &[129]);     // 33 => [129]
            map.insert(34, &[257]);     // 34 => [257]
            map.insert(36, &[513]);     // 36 => [513]
            map.insert(40, &[1025]);     // 40 => [1025]
            map.insert(65, &[2049]);     // 65 => [2049]
            map.insert(71, &[4097]);     // 71 => [4097]
            map.insert(75, &[8193]);     // 75 => [8193]
            map.insert(51, &[16385]);     // 51 => [16385]
            map.insert(18, &[32769]);     // 18 => [32769]
            map.insert(113, &[65537]);     // 113 => [65537]
            map.insert(114, &[131073]);     // 114 => [131073]
            map.insert(116, &[262145]);     // 116 => [262145]
            map.insert(120, &[524289]);     // 120 => [524289]
            map.insert(66, &[1048577]);     // 66 => [1048577]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(20, &[34]);     // 20 => [34]
            map.insert(24, &[66]);     // 24 => [66]
            map.insert(39, &[514]);     // 39 => [514]
            map.insert(43, &[1026]);     // 43 => [1026]
            map.insert(68, &[4098]);     // 68 => [4098]
            map.insert(72, &[8194]);     // 72 => [8194]
            map.insert(48, &[16386]);     // 48 => [16386]
            map.insert(119, &[262146]);     // 119 => [262146]
            map.insert(123, &[524290]);     // 123 => [524290]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(30, &[68]);     // 30 => [68]
            map.insert(45, &[1028]);     // 45 => [1028]
            map.insert(78, &[8196]);     // 78 => [8196]
            map.insert(54, &[16388]);     // 54 => [16388]
            map.insert(125, &[524292]);     // 125 => [524292]
            map.insert(58, &[16392]);     // 58 => [16392]
            map.insert(53, &[528]);     // 53 => [528]
            map.insert(57, &[1040]);     // 57 => [1040]
            map.insert(80, &[2064]);     // 80 => [2064]
            map.insert(86, &[4112]);     // 86 => [4112]
            map.insert(90, &[8208]);     // 90 => [8208]
            map.insert(96, &[65552]);     // 96 => [65552]
            map.insert(99, &[131088]);     // 99 => [131088]
            map.insert(101, &[262160]);     // 101 => [262160]
            map.insert(105, &[524304]);     // 105 => [524304]
            map.insert(83, &[1048592]);     // 83 => [1048592]
            map.insert(63, &[1056]);     // 63 => [1056]
            map.insert(92, &[8224]);     // 92 => [8224]
            map.insert(102, &[65568]);     // 102 => [65568]
            map.insert(111, &[524320]);     // 111 => [524320]
            map.insert(85, &[1048608]);     // 85 => [1048608]
            map.insert(106, &[65600]);     // 106 => [65600]
            map.insert(89, &[1048640]);     // 89 => [1048640]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(11, &[11]);     // 11 => [11]
            map.insert(21, &[35]);     // 21 => [35]
            map.insert(25, &[67]);     // 25 => [67]
            map.insert(38, &[515]);     // 38 => [515]
            map.insert(42, &[1027]);     // 42 => [1027]
            map.insert(69, &[4099]);     // 69 => [4099]
            map.insert(73, &[8195]);     // 73 => [8195]
            map.insert(49, &[16387]);     // 49 => [16387]
            map.insert(118, &[262147]);     // 118 => [262147]
            map.insert(122, &[524291]);     // 122 => [524291]
            map.insert(13, &[13]);     // 13 => [13]
            map.insert(31, &[69]);     // 31 => [69]
            map.insert(44, &[1029]);     // 44 => [1029]
            map.insert(79, &[8197]);     // 79 => [8197]
            map.insert(55, &[16389]);     // 55 => [16389]
            map.insert(124, &[524293]);     // 124 => [524293]
            map.insert(59, &[16393]);     // 59 => [16393]
            map.insert(52, &[529]);     // 52 => [529]
            map.insert(56, &[1041]);     // 56 => [1041]
            map.insert(81, &[2065]);     // 81 => [2065]
            map.insert(87, &[4113]);     // 87 => [4113]
            map.insert(91, &[8209]);     // 91 => [8209]
            map.insert(97, &[65553]);     // 97 => [65553]
            map.insert(98, &[131089]);     // 98 => [131089]
            map.insert(100, &[262161]);     // 100 => [262161]
            map.insert(104, &[524305]);     // 104 => [524305]
            map.insert(82, &[1048593]);     // 82 => [1048593]
            map.insert(62, &[1057]);     // 62 => [1057]
            map.insert(93, &[8225]);     // 93 => [8225]
            map.insert(103, &[65569]);     // 103 => [65569]
            map.insert(110, &[524321]);     // 110 => [524321]
            map.insert(84, &[1048609]);     // 84 => [1048609]
            map.insert(107, &[65601]);     // 107 => [65601]
            map.insert(88, &[1048641]);     // 88 => [1048641]
            map.insert(14, &[14]);     // 14 => [14]
            map.insert(28, &[70]);     // 28 => [70]
            map.insert(47, &[1030]);     // 47 => [1030]
            map.insert(76, &[8198]);     // 76 => [8198]
            map.insert(127, &[524294]);     // 127 => [524294]
            map.insert(61, &[1058]);     // 61 => [1058]
            map.insert(94, &[8226]);     // 94 => [8226]
            map.insert(109, &[524322]);     // 109 => [524322]
            map.insert(15, &[15]);     // 15 => [15]
            map.insert(29, &[71]);     // 29 => [71]
            map.insert(46, &[1031]);     // 46 => [1031]
            map.insert(77, &[8199]);     // 77 => [8199]
            map.insert(126, &[524295]);     // 126 => [524295]
            map.insert(60, &[1059]);     // 60 => [1059]
            map.insert(95, &[8227]);     // 95 => [8227]
            map.insert(108, &[524323]);     // 108 => [524323]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode21_14 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode21_14 {
    fn name(&self) -> String {
        "[21, 14] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        21
    }

    fn dimension(&self) -> usize {
        14
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
        codeword.truncate(14);
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
        let code = GuavaCode21_14.generator_matrix();
        assert_eq!(code.ncols(), 21);
        assert_eq!(code.nrows(), 14);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode21_14;
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
            let code = GuavaCode21_14;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, false, true, true, true, false, true, false, false, true, false, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, false, false, true, true, false, true, false, false, true, false, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_14;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, false, false, true, false, false, false, false, true, false, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, false, false, false, true, false, false, false, false, true, false, false, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_14;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, true, false, false, true, false, false, false, false, false, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, true, false, false, true, false, false, false, false, false, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_14;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, false, true, false, true, false, true, false, true, true, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, false, true, false, true, false, true, false, true, true, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_14;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, true, true, true, true, true, false, false, false, true, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, true, true, true, true, false, false, false, false, true, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_14;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, true, false, false, false, true, true, true, false, true, true, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, true, false, false, false, true, true, true, false, true, true, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_14;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, false, true, false, false, false, true, false, false, true, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, false, true, false, false, false, true, false, false, true, true, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_14;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, false, true, true, true, true, false, true, false, false, false, false, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, true, true, true, true, false, true, false, false, false, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_14;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, true, true, true, true, true, true, true, false, true, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, true, true, true, true, true, true, true, true, false, true, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_14;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, true, true, false, true, false, false, true, true, true, false, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, true, true, true, false, true, false, false, true, false, true, false, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_14;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, true, true, true, true, false, false, true, true, true, true, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, true, true, true, true, false, false, true, true, true, true, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_14;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, true, false, false, true, true, true, false, true, true, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, false, true, false, false, true, true, true, false, true, true, false, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_14;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, true, false, true, true, false, false, false, true, true, true, false, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, true, false, true, true, false, false, false, true, true, true, false, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_14;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, false, false, false, true, false, false, true, false, false, false, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, true, false, false, true, false, false, true, false, false, false, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_14;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, false, true, true, false, true, true, true, true, false, false, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, false, false, true, false, true, true, true, true, false, false, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_14;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, false, true, false, true, false, false, false, true, true, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, false, true, false, true, false, false, false, true, true, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_14;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, false, true, false, true, false, true, true, true, false, true, true, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, true, false, true, false, true, true, true, true, true, true, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_14;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, false, true, true, false, true, true, true, false, true, false, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, false, true, false, true, true, true, false, true, false, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_14;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, true, false, true, false, false, true, true, false, false, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, false, true, false, true, false, false, true, true, false, false, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode21_14;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, true, true, false, false, false, false, true, true, false, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, true, true, false, false, true, false, true, true, false, true, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
