use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[19, 12]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode19_12;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 331777 ],
                &[ 397314 ],
                &[ 57348 ],
                &[ 90120 ],
                &[ 155664 ],
                &[ 286752 ],
                &[ 303168 ],
                &[ 336000 ],
                &[ 401664 ],
                &[ 311808 ],
                &[ 345088 ],
                &[ 411648 ],
                
            ], 19));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 137489 ],
                &[ 133394 ],
                &[ 60804 ],
                &[ 204184 ],
                &[ 290848 ],
                &[ 500160 ],
                &[ 511488 ],
                
            ], 19));
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
            map.insert(32, &[64]);     // 32 => [64]
            map.insert(44, &[128]);     // 44 => [128]
            map.insert(47, &[256]);     // 47 => [256]
            map.insert(64, &[512]);     // 64 => [512]
            map.insert(76, &[1024]);     // 76 => [1024]
            map.insert(79, &[2048]);     // 79 => [2048]
            map.insert(25, &[4096]);     // 25 => [4096]
            map.insert(52, &[8192]);     // 52 => [8192]
            map.insert(84, &[16384]);     // 84 => [16384]
            map.insert(100, &[32768]);     // 100 => [32768]
            map.insert(104, &[65536]);     // 104 => [65536]
            map.insert(107, &[131072]);     // 107 => [131072]
            map.insert(112, &[262144]);     // 112 => [262144]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(10, &[17]);     // 10 => [17]
            map.insert(17, &[33]);     // 17 => [33]
            map.insert(33, &[65]);     // 33 => [65]
            map.insert(45, &[129]);     // 45 => [129]
            map.insert(46, &[257]);     // 46 => [257]
            map.insert(65, &[513]);     // 65 => [513]
            map.insert(77, &[1025]);     // 77 => [1025]
            map.insert(78, &[2049]);     // 78 => [2049]
            map.insert(24, &[4097]);     // 24 => [4097]
            map.insert(53, &[8193]);     // 53 => [8193]
            map.insert(85, &[16385]);     // 85 => [16385]
            map.insert(101, &[32769]);     // 101 => [32769]
            map.insert(105, &[65537]);     // 105 => [65537]
            map.insert(106, &[131073]);     // 106 => [131073]
            map.insert(113, &[262145]);     // 113 => [262145]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(18, &[34]);     // 18 => [34]
            map.insert(34, &[66]);     // 34 => [66]
            map.insert(66, &[514]);     // 66 => [514]
            map.insert(27, &[4098]);     // 27 => [4098]
            map.insert(54, &[8194]);     // 54 => [8194]
            map.insert(86, &[16386]);     // 86 => [16386]
            map.insert(102, &[32770]);     // 102 => [32770]
            map.insert(114, &[262146]);     // 114 => [262146]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(15, &[20]);     // 15 => [20]
            map.insert(20, &[36]);     // 20 => [36]
            map.insert(36, &[68]);     // 36 => [68]
            map.insert(40, &[132]);     // 40 => [132]
            map.insert(43, &[260]);     // 43 => [260]
            map.insert(68, &[516]);     // 68 => [516]
            map.insert(72, &[1028]);     // 72 => [1028]
            map.insert(75, &[2052]);     // 75 => [2052]
            map.insert(29, &[4100]);     // 29 => [4100]
            map.insert(48, &[8196]);     // 48 => [8196]
            map.insert(80, &[16388]);     // 80 => [16388]
            map.insert(96, &[32772]);     // 96 => [32772]
            map.insert(108, &[65540]);     // 108 => [65540]
            map.insert(111, &[131076]);     // 111 => [131076]
            map.insert(116, &[262148]);     // 116 => [262148]
            map.insert(39, &[264]);     // 39 => [264]
            map.insert(71, &[2056]);     // 71 => [2056]
            map.insert(60, &[8200]);     // 60 => [8200]
            map.insert(92, &[16392]);     // 92 => [16392]
            map.insert(99, &[131080]);     // 99 => [131080]
            map.insert(120, &[262152]);     // 120 => [262152]
            map.insert(63, &[8208]);     // 63 => [8208]
            map.insert(95, &[16400]);     // 95 => [16400]
            map.insert(123, &[262160]);     // 123 => [262160]
            map.insert(57, &[4160]);     // 57 => [4160]
            map.insert(89, &[4608]);     // 89 => [4608]
            map.insert(125, &[36864]);     // 125 => [36864]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(19, &[35]);     // 19 => [35]
            map.insert(35, &[67]);     // 35 => [67]
            map.insert(67, &[515]);     // 67 => [515]
            map.insert(26, &[4099]);     // 26 => [4099]
            map.insert(55, &[8195]);     // 55 => [8195]
            map.insert(87, &[16387]);     // 87 => [16387]
            map.insert(103, &[32771]);     // 103 => [32771]
            map.insert(115, &[262147]);     // 115 => [262147]
            map.insert(13, &[13]);     // 13 => [13]
            map.insert(14, &[21]);     // 14 => [21]
            map.insert(21, &[37]);     // 21 => [37]
            map.insert(37, &[69]);     // 37 => [69]
            map.insert(41, &[133]);     // 41 => [133]
            map.insert(42, &[261]);     // 42 => [261]
            map.insert(69, &[517]);     // 69 => [517]
            map.insert(73, &[1029]);     // 73 => [1029]
            map.insert(74, &[2053]);     // 74 => [2053]
            map.insert(28, &[4101]);     // 28 => [4101]
            map.insert(49, &[8197]);     // 49 => [8197]
            map.insert(81, &[16389]);     // 81 => [16389]
            map.insert(97, &[32773]);     // 97 => [32773]
            map.insert(109, &[65541]);     // 109 => [65541]
            map.insert(110, &[131077]);     // 110 => [131077]
            map.insert(117, &[262149]);     // 117 => [262149]
            map.insert(38, &[265]);     // 38 => [265]
            map.insert(70, &[2057]);     // 70 => [2057]
            map.insert(61, &[8201]);     // 61 => [8201]
            map.insert(93, &[16393]);     // 93 => [16393]
            map.insert(98, &[131081]);     // 98 => [131081]
            map.insert(121, &[262153]);     // 121 => [262153]
            map.insert(62, &[8209]);     // 62 => [8209]
            map.insert(94, &[16401]);     // 94 => [16401]
            map.insert(122, &[262161]);     // 122 => [262161]
            map.insert(56, &[4161]);     // 56 => [4161]
            map.insert(88, &[4609]);     // 88 => [4609]
            map.insert(124, &[36865]);     // 124 => [36865]
            map.insert(22, &[38]);     // 22 => [38]
            map.insert(31, &[4102]);     // 31 => [4102]
            map.insert(50, &[8198]);     // 50 => [8198]
            map.insert(82, &[16390]);     // 82 => [16390]
            map.insert(118, &[262150]);     // 118 => [262150]
            map.insert(59, &[4162]);     // 59 => [4162]
            map.insert(91, &[4610]);     // 91 => [4610]
            map.insert(127, &[36866]);     // 127 => [36866]
            map.insert(23, &[39]);     // 23 => [39]
            map.insert(30, &[4103]);     // 30 => [4103]
            map.insert(51, &[8199]);     // 51 => [8199]
            map.insert(83, &[16391]);     // 83 => [16391]
            map.insert(119, &[262151]);     // 119 => [262151]
            map.insert(58, &[4163]);     // 58 => [4163]
            map.insert(90, &[4611]);     // 90 => [4611]
            map.insert(126, &[36867]);     // 126 => [36867]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode19_12 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode19_12 {
    fn name(&self) -> String {
        "[19, 12] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        19
    }

    fn dimension(&self) -> usize {
        12
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
        let mut error = BinVector::with_capacity(19);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 19 / 64 + if 19 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(19) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(12);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[19 / 64] & !((1 << 19) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode19_12.generator_matrix();
        assert_eq!(code.ncols(), 19);
        assert_eq!(code.nrows(), 12);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode19_12;
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
            let code = GuavaCode19_12;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, true, false, true, false, false, true, false, true, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, true, true, true, false, false, true, false, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_12;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, false, false, false, false, false, true, false, true, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, false, false, false, false, false, true, false, true, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_12;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, false, true, false, false, false, true, true, true, true, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, true, false, false, false, true, true, false, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_12;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, false, true, false, true, false, false, true, false, true, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, false, true, false, true, false, false, true, false, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_12;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, true, true, true, false, true, false, false, false, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, false, true, true, false, true, false, false, false, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_12;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, true, false, false, true, false, true, true, false, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, false, false, false, false, true, true, false, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_12;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, false, true, true, false, false, false, true, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, false, false, true, false, false, false, false, true, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_12;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, false, false, true, false, true, true, false, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, false, false, true, false, true, true, true, false, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_12;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, true, false, false, false, false, false, true, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, true, true, false, true, false, false, false, true, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_12;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, false, true, true, true, false, false, false, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, false, false, true, true, true, false, false, false, true, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_12;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, true, true, true, true, false, true, false, true, true, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, true, true, true, true, false, true, false, true, true, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_12;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, false, false, true, false, false, true, false, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, false, false, true, false, false, true, false, false, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_12;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, true, false, true, true, true, false, false, false, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, false, true, true, true, false, false, false, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_12;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, true, true, false, true, true, false, false, false, true, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, false, true, true, false, true, true, false, false, false, true, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_12;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, false, false, false, true, false, true, true, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, true, false, false, false, true, false, true, true, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_12;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, false, false, false, true, true, false, false, false, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, true, false, false, true, true, false, false, false, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_12;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, false, false, true, false, true, true, true, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, true, false, true, false, true, true, true, true, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_12;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, true, false, true, false, false, false, false, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, true, true, false, true, false, false, false, false, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_12;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, true, true, true, true, true, false, false, false, false, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, true, true, true, true, false, false, false, false, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode19_12;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, false, false, true, true, true, false, false, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, false, false, true, true, true, false, false, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
