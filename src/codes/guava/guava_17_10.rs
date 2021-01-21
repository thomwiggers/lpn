use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[17, 10]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode17_10;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 14337 ],
                &[ 22530 ],
                &[ 38916 ],
                &[ 71688 ],
                &[ 75792 ],
                &[ 84000 ],
                &[ 100416 ],
                &[ 77952 ],
                &[ 86272 ],
                &[ 102912 ],
                
            ], 17));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 15201 ],
                &[ 16674 ],
                &[ 33348 ],
                &[ 71688 ],
                &[ 125040 ],
                &[ 127872 ],
                &[ 1024 ],
                
            ], 17));
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
            map.insert(37, &[512]);     // 37 => [512]
            map.insert(64, &[1024]);     // 64 => [1024]
            map.insert(25, &[2048]);     // 25 => [2048]
            map.insert(41, &[4096]);     // 41 => [4096]
            map.insert(49, &[8192]);     // 49 => [8192]
            map.insert(50, &[16384]);     // 50 => [16384]
            map.insert(52, &[32768]);     // 52 => [32768]
            map.insert(56, &[65536]);     // 56 => [65536]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(18, &[33]);     // 18 => [33]
            map.insert(20, &[65]);     // 20 => [65]
            map.insert(33, &[129]);     // 33 => [129]
            map.insert(34, &[257]);     // 34 => [257]
            map.insert(36, &[513]);     // 36 => [513]
            map.insert(65, &[1025]);     // 65 => [1025]
            map.insert(24, &[2049]);     // 24 => [2049]
            map.insert(40, &[4097]);     // 40 => [4097]
            map.insert(48, &[8193]);     // 48 => [8193]
            map.insert(51, &[16385]);     // 51 => [16385]
            map.insert(53, &[32769]);     // 53 => [32769]
            map.insert(57, &[65537]);     // 57 => [65537]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(23, &[66]);     // 23 => [66]
            map.insert(39, &[514]);     // 39 => [514]
            map.insert(66, &[1026]);     // 66 => [1026]
            map.insert(27, &[2050]);     // 27 => [2050]
            map.insert(43, &[4098]);     // 43 => [4098]
            map.insert(54, &[32770]);     // 54 => [32770]
            map.insert(58, &[65538]);     // 58 => [65538]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(68, &[1028]);     // 68 => [1028]
            map.insert(29, &[2052]);     // 29 => [2052]
            map.insert(45, &[4100]);     // 45 => [4100]
            map.insert(60, &[65540]);     // 60 => [65540]
            map.insert(72, &[1032]);     // 72 => [1032]
            map.insert(80, &[1040]);     // 80 => [1040]
            map.insert(83, &[1056]);     // 83 => [1056]
            map.insert(85, &[1088]);     // 85 => [1088]
            map.insert(96, &[1152]);     // 96 => [1152]
            map.insert(99, &[1280]);     // 99 => [1280]
            map.insert(101, &[1536]);     // 101 => [1536]
            map.insert(89, &[3072]);     // 89 => [3072]
            map.insert(105, &[5120]);     // 105 => [5120]
            map.insert(113, &[9216]);     // 113 => [9216]
            map.insert(114, &[17408]);     // 114 => [17408]
            map.insert(116, &[33792]);     // 116 => [33792]
            map.insert(120, &[66560]);     // 120 => [66560]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(11, &[11]);     // 11 => [11]
            map.insert(22, &[67]);     // 22 => [67]
            map.insert(38, &[515]);     // 38 => [515]
            map.insert(67, &[1027]);     // 67 => [1027]
            map.insert(26, &[2051]);     // 26 => [2051]
            map.insert(42, &[4099]);     // 42 => [4099]
            map.insert(55, &[32771]);     // 55 => [32771]
            map.insert(59, &[65539]);     // 59 => [65539]
            map.insert(13, &[13]);     // 13 => [13]
            map.insert(69, &[1029]);     // 69 => [1029]
            map.insert(28, &[2053]);     // 28 => [2053]
            map.insert(44, &[4101]);     // 44 => [4101]
            map.insert(61, &[65541]);     // 61 => [65541]
            map.insert(73, &[1033]);     // 73 => [1033]
            map.insert(81, &[1041]);     // 81 => [1041]
            map.insert(82, &[1057]);     // 82 => [1057]
            map.insert(84, &[1089]);     // 84 => [1089]
            map.insert(97, &[1153]);     // 97 => [1153]
            map.insert(98, &[1281]);     // 98 => [1281]
            map.insert(100, &[1537]);     // 100 => [1537]
            map.insert(88, &[3073]);     // 88 => [3073]
            map.insert(104, &[5121]);     // 104 => [5121]
            map.insert(112, &[9217]);     // 112 => [9217]
            map.insert(115, &[17409]);     // 115 => [17409]
            map.insert(117, &[33793]);     // 117 => [33793]
            map.insert(121, &[66561]);     // 121 => [66561]
            map.insert(14, &[14]);     // 14 => [14]
            map.insert(70, &[1030]);     // 70 => [1030]
            map.insert(31, &[2054]);     // 31 => [2054]
            map.insert(47, &[4102]);     // 47 => [4102]
            map.insert(62, &[65542]);     // 62 => [65542]
            map.insert(74, &[1034]);     // 74 => [1034]
            map.insert(87, &[1090]);     // 87 => [1090]
            map.insert(103, &[1538]);     // 103 => [1538]
            map.insert(91, &[3074]);     // 91 => [3074]
            map.insert(107, &[5122]);     // 107 => [5122]
            map.insert(118, &[33794]);     // 118 => [33794]
            map.insert(122, &[66562]);     // 122 => [66562]
            map.insert(76, &[1036]);     // 76 => [1036]
            map.insert(93, &[3076]);     // 93 => [3076]
            map.insert(109, &[5124]);     // 109 => [5124]
            map.insert(124, &[66564]);     // 124 => [66564]
            map.insert(15, &[15]);     // 15 => [15]
            map.insert(71, &[1031]);     // 71 => [1031]
            map.insert(30, &[2055]);     // 30 => [2055]
            map.insert(46, &[4103]);     // 46 => [4103]
            map.insert(63, &[65543]);     // 63 => [65543]
            map.insert(75, &[1035]);     // 75 => [1035]
            map.insert(86, &[1091]);     // 86 => [1091]
            map.insert(102, &[1539]);     // 102 => [1539]
            map.insert(90, &[3075]);     // 90 => [3075]
            map.insert(106, &[5123]);     // 106 => [5123]
            map.insert(119, &[33795]);     // 119 => [33795]
            map.insert(123, &[66563]);     // 123 => [66563]
            map.insert(77, &[1037]);     // 77 => [1037]
            map.insert(92, &[3077]);     // 92 => [3077]
            map.insert(108, &[5125]);     // 108 => [5125]
            map.insert(125, &[66565]);     // 125 => [66565]
            map.insert(78, &[1038]);     // 78 => [1038]
            map.insert(95, &[3078]);     // 95 => [3078]
            map.insert(111, &[5126]);     // 111 => [5126]
            map.insert(126, &[66566]);     // 126 => [66566]
            map.insert(79, &[1039]);     // 79 => [1039]
            map.insert(94, &[3079]);     // 94 => [3079]
            map.insert(110, &[5127]);     // 110 => [5127]
            map.insert(127, &[66567]);     // 127 => [66567]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode17_10 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode17_10 {
    fn name(&self) -> String {
        "[17, 10] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        17
    }

    fn dimension(&self) -> usize {
        10
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
        let mut error = BinVector::with_capacity(17);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 17 / 64 + if 17 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(17) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(10);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[17 / 64] & !((1 << 17) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode17_10.generator_matrix();
        assert_eq!(code.ncols(), 17);
        assert_eq!(code.nrows(), 10);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode17_10;
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
            let code = GuavaCode17_10;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, true, false, true, false, false, false, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, true, false, false, false, false, false, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_10;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, true, false, true, true, true, true, true, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, true, true, true, true, false, true, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_10;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, false, false, true, false, true, true, true, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, false, true, false, true, false, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_10;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, true, true, false, false, true, true, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, true, true, false, false, false, true, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_10;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, true, true, true, false, true, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, false, true, true, false, false, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_10;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, false, true, true, false, true, true, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, false, true, true, false, false, true, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_10;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, false, true, false, true, true, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, false, true, false, true, false, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_10;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, false, true, true, false, true, true, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, true, true, false, true, false, true, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_10;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, false, true, false, false, false, false, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, false, true, false, false, false, false, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_10;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, false, false, true, false, false, true, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_10;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, false, true, false, false, false, true, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, true, false, false, false, false, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_10;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, false, false, true, false, true, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, false, false, true, false, false, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_10;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, true, true, false, false, true, true, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, true, true, false, false, false, false, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_10;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, true, true, true, true, true, false, false, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, true, true, true, true, true, false, false, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_10;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, false, false, true, true, false, true, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, false, false, false, true, true, false, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_10;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, false, true, false, false, false, false, true, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, true, false, false, false, false, true, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_10;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, true, false, false, true, true, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, true, false, false, true, false, false, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_10;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, false, false, false, false, true, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, false, false, false, false, false, true, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_10;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, true, true, true, true, true, false, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, true, true, true, true, false, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode17_10;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, true, true, false, false, false, false, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, true, false, false, false, false, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, false, false, true, true, true, false, false, false ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
