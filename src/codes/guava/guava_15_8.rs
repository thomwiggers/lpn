use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[15, 8]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode15_8;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 29441 ],
                &[ 17154 ],
                &[ 17668 ],
                &[ 18696 ],
                &[ 20752 ],
                &[ 24864 ],
                &[ 10816 ],
                &[ 12928 ],
                
            ], 15));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 24097 ],
                &[ 8738 ],
                &[ 1028 ],
                &[ 30344 ],
                &[ 20144 ],
                &[ 32448 ],
                &[ 16640 ],
                
            ], 15));
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
            map.insert(32, &[64]);     // 32 => [64]
            map.insert(56, &[128]);     // 56 => [128]
            map.insert(64, &[256]);     // 64 => [256]
            map.insert(59, &[512]);     // 59 => [512]
            map.insert(61, &[1024]);     // 61 => [1024]
            map.insert(49, &[2048]);     // 49 => [2048]
            map.insert(41, &[4096]);     // 41 => [4096]
            map.insert(42, &[8192]);     // 42 => [8192]
            map.insert(121, &[16384]);     // 121 => [16384]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(18, &[33]);     // 18 => [33]
            map.insert(33, &[65]);     // 33 => [65]
            map.insert(57, &[129]);     // 57 => [129]
            map.insert(65, &[257]);     // 65 => [257]
            map.insert(58, &[513]);     // 58 => [513]
            map.insert(60, &[1025]);     // 60 => [1025]
            map.insert(48, &[2049]);     // 48 => [2049]
            map.insert(40, &[4097]);     // 40 => [4097]
            map.insert(43, &[8193]);     // 43 => [8193]
            map.insert(120, &[16385]);     // 120 => [16385]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(34, &[66]);     // 34 => [66]
            map.insert(66, &[258]);     // 66 => [258]
            map.insert(63, &[1026]);     // 63 => [1026]
            map.insert(51, &[2050]);     // 51 => [2050]
            map.insert(123, &[16386]);     // 123 => [16386]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(20, &[20]);     // 20 => [20]
            map.insert(23, &[36]);     // 23 => [36]
            map.insert(36, &[68]);     // 36 => [68]
            map.insert(68, &[260]);     // 68 => [260]
            map.insert(53, &[2052]);     // 53 => [2052]
            map.insert(45, &[4100]);     // 45 => [4100]
            map.insert(46, &[8196]);     // 46 => [8196]
            map.insert(125, &[16388]);     // 125 => [16388]
            map.insert(24, &[24]);     // 24 => [24]
            map.insert(27, &[40]);     // 27 => [40]
            map.insert(72, &[264]);     // 72 => [264]
            map.insert(113, &[16392]);     // 113 => [16392]
            map.insert(80, &[272]);     // 80 => [272]
            map.insert(105, &[16400]);     // 105 => [16400]
            map.insert(83, &[288]);     // 83 => [288]
            map.insert(106, &[16416]);     // 106 => [16416]
            map.insert(96, &[320]);     // 96 => [320]
            map.insert(29, &[1088]);     // 29 => [1088]
            map.insert(89, &[16448]);     // 89 => [16448]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(11, &[11]);     // 11 => [11]
            map.insert(35, &[67]);     // 35 => [67]
            map.insert(67, &[259]);     // 67 => [259]
            map.insert(62, &[1027]);     // 62 => [1027]
            map.insert(50, &[2051]);     // 50 => [2051]
            map.insert(122, &[16387]);     // 122 => [16387]
            map.insert(13, &[13]);     // 13 => [13]
            map.insert(21, &[21]);     // 21 => [21]
            map.insert(22, &[37]);     // 22 => [37]
            map.insert(37, &[69]);     // 37 => [69]
            map.insert(69, &[261]);     // 69 => [261]
            map.insert(52, &[2053]);     // 52 => [2053]
            map.insert(44, &[4101]);     // 44 => [4101]
            map.insert(47, &[8197]);     // 47 => [8197]
            map.insert(124, &[16389]);     // 124 => [16389]
            map.insert(25, &[25]);     // 25 => [25]
            map.insert(26, &[41]);     // 26 => [41]
            map.insert(73, &[265]);     // 73 => [265]
            map.insert(112, &[16393]);     // 112 => [16393]
            map.insert(81, &[273]);     // 81 => [273]
            map.insert(104, &[16401]);     // 104 => [16401]
            map.insert(82, &[289]);     // 82 => [289]
            map.insert(107, &[16417]);     // 107 => [16417]
            map.insert(97, &[321]);     // 97 => [321]
            map.insert(28, &[1089]);     // 28 => [1089]
            map.insert(88, &[16449]);     // 88 => [16449]
            map.insert(14, &[14]);     // 14 => [14]
            map.insert(38, &[70]);     // 38 => [70]
            map.insert(70, &[262]);     // 70 => [262]
            map.insert(55, &[2054]);     // 55 => [2054]
            map.insert(127, &[16390]);     // 127 => [16390]
            map.insert(74, &[266]);     // 74 => [266]
            map.insert(115, &[16394]);     // 115 => [16394]
            map.insert(98, &[322]);     // 98 => [322]
            map.insert(31, &[1090]);     // 31 => [1090]
            map.insert(91, &[16450]);     // 91 => [16450]
            map.insert(76, &[268]);     // 76 => [268]
            map.insert(117, &[16396]);     // 117 => [16396]
            map.insert(84, &[276]);     // 84 => [276]
            map.insert(109, &[16404]);     // 109 => [16404]
            map.insert(87, &[292]);     // 87 => [292]
            map.insert(110, &[16420]);     // 110 => [16420]
            map.insert(100, &[324]);     // 100 => [324]
            map.insert(93, &[16452]);     // 93 => [16452]
            map.insert(15, &[15]);     // 15 => [15]
            map.insert(39, &[71]);     // 39 => [71]
            map.insert(71, &[263]);     // 71 => [263]
            map.insert(54, &[2055]);     // 54 => [2055]
            map.insert(126, &[16391]);     // 126 => [16391]
            map.insert(75, &[267]);     // 75 => [267]
            map.insert(114, &[16395]);     // 114 => [16395]
            map.insert(99, &[323]);     // 99 => [323]
            map.insert(30, &[1091]);     // 30 => [1091]
            map.insert(90, &[16451]);     // 90 => [16451]
            map.insert(77, &[269]);     // 77 => [269]
            map.insert(116, &[16397]);     // 116 => [16397]
            map.insert(85, &[277]);     // 85 => [277]
            map.insert(108, &[16405]);     // 108 => [16405]
            map.insert(86, &[293]);     // 86 => [293]
            map.insert(111, &[16421]);     // 111 => [16421]
            map.insert(101, &[325]);     // 101 => [325]
            map.insert(92, &[16453]);     // 92 => [16453]
            map.insert(78, &[270]);     // 78 => [270]
            map.insert(119, &[16398]);     // 119 => [16398]
            map.insert(102, &[326]);     // 102 => [326]
            map.insert(95, &[16454]);     // 95 => [16454]
            map.insert(79, &[271]);     // 79 => [271]
            map.insert(118, &[16399]);     // 118 => [16399]
            map.insert(103, &[327]);     // 103 => [327]
            map.insert(94, &[16455]);     // 94 => [16455]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode15_8 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode15_8 {
    fn name(&self) -> String {
        "[15, 8] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        15
    }

    fn dimension(&self) -> usize {
        8
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
        let mut error = BinVector::with_capacity(15);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 15 / 64 + if 15 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(15) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(8);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[15 / 64] & !((1 << 15) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode15_8.generator_matrix();
        assert_eq!(code.ncols(), 15);
        assert_eq!(code.nrows(), 8);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode15_8;
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
            let code = GuavaCode15_8;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, true, false, false, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, true, true, false, true, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_8;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, true, true, false, true, false, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, true, true, false, true, false, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_8;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, true, true, true, false, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, true, true, false, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_8;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, true, false, true, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, false, true, false, true, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_8;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, false, false, true, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, false, false, true, true, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_8;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, true, false, true, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, false, true, false, false, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_8;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, true, false, false, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, true, true, false, true, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_8;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, false, false, true, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, true, false, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_8;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, false, true, true, false, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, true, true, false, true, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_8;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, false, false, true, false, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, false, false, false, true, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_8;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, true, false, false, false, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, true, false, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_8;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, true, false, false, true, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, true, false, false, true, false, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_8;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, true, true, true, true, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, false, true, true, true, true, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_8;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, false, false, false, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, true, false, false, false, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_8;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, false, true, true, false, true, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, false, true, true, true, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_8;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, false, true, false, false, true, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, false, false, false, true, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_8;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, false, true, true, true, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, true, true, true, true, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_8;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, true, true, false, true, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, true, false, false, true, true, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_8;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, false, false, true, true, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, false, false, true, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode15_8;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, false, false, false, true, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, true, false, false, false, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, true, true, false, false, true, true, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
