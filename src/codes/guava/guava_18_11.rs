use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[18, 11]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode18_11;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 198657 ],
                &[ 28674 ],
                &[ 45060 ],
                &[ 77832 ],
                &[ 143376 ],
                &[ 151584 ],
                &[ 168000 ],
                &[ 200832 ],
                &[ 155904 ],
                &[ 172544 ],
                &[ 205824 ],
                
            ], 18));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 2049 ],
                &[ 30402 ],
                &[ 33348 ],
                &[ 68744 ],
                &[ 145424 ],
                &[ 250080 ],
                &[ 255744 ],
                
            ], 18));
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
            map.insert(38, &[64]);     // 38 => [64]
            map.insert(42, &[128]);     // 42 => [128]
            map.insert(64, &[256]);     // 64 => [256]
            map.insert(70, &[512]);     // 70 => [512]
            map.insert(74, &[1024]);     // 74 => [1024]
            map.insert(25, &[2048]);     // 25 => [2048]
            map.insert(50, &[4096]);     // 50 => [4096]
            map.insert(82, &[8192]);     // 82 => [8192]
            map.insert(98, &[16384]);     // 98 => [16384]
            map.insert(100, &[32768]);     // 100 => [32768]
            map.insert(104, &[65536]);     // 104 => [65536]
            map.insert(112, &[131072]);     // 112 => [131072]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(33, &[33]);     // 33 => [33]
            map.insert(39, &[65]);     // 39 => [65]
            map.insert(43, &[129]);     // 43 => [129]
            map.insert(65, &[257]);     // 65 => [257]
            map.insert(71, &[513]);     // 71 => [513]
            map.insert(75, &[1025]);     // 75 => [1025]
            map.insert(24, &[2049]);     // 24 => [2049]
            map.insert(51, &[4097]);     // 51 => [4097]
            map.insert(83, &[8193]);     // 83 => [8193]
            map.insert(99, &[16385]);     // 99 => [16385]
            map.insert(101, &[32769]);     // 101 => [32769]
            map.insert(105, &[65537]);     // 105 => [65537]
            map.insert(113, &[131073]);     // 113 => [131073]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(18, &[18]);     // 18 => [18]
            map.insert(34, &[34]);     // 34 => [34]
            map.insert(36, &[66]);     // 36 => [66]
            map.insert(40, &[130]);     // 40 => [130]
            map.insert(66, &[258]);     // 66 => [258]
            map.insert(68, &[514]);     // 68 => [514]
            map.insert(72, &[1026]);     // 72 => [1026]
            map.insert(27, &[2050]);     // 27 => [2050]
            map.insert(48, &[4098]);     // 48 => [4098]
            map.insert(80, &[8194]);     // 80 => [8194]
            map.insert(96, &[16386]);     // 96 => [16386]
            map.insert(102, &[32770]);     // 102 => [32770]
            map.insert(106, &[65538]);     // 106 => [65538]
            map.insert(114, &[131074]);     // 114 => [131074]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(20, &[20]);     // 20 => [20]
            map.insert(46, &[132]);     // 46 => [132]
            map.insert(78, &[1028]);     // 78 => [1028]
            map.insert(29, &[2052]);     // 29 => [2052]
            map.insert(54, &[4100]);     // 54 => [4100]
            map.insert(86, &[8196]);     // 86 => [8196]
            map.insert(108, &[65540]);     // 108 => [65540]
            map.insert(116, &[131076]);     // 116 => [131076]
            map.insert(58, &[4104]);     // 58 => [4104]
            map.insert(90, &[8200]);     // 90 => [8200]
            map.insert(120, &[131080]);     // 120 => [131080]
            map.insert(57, &[2080]);     // 57 => [2080]
            map.insert(63, &[2112]);     // 63 => [2112]
            map.insert(89, &[2304]);     // 89 => [2304]
            map.insert(95, &[2560]);     // 95 => [2560]
            map.insert(123, &[18432]);     // 123 => [18432]
            map.insert(125, &[34816]);     // 125 => [34816]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(11, &[11]);     // 11 => [11]
            map.insert(19, &[19]);     // 19 => [19]
            map.insert(35, &[35]);     // 35 => [35]
            map.insert(37, &[67]);     // 37 => [67]
            map.insert(41, &[131]);     // 41 => [131]
            map.insert(67, &[259]);     // 67 => [259]
            map.insert(69, &[515]);     // 69 => [515]
            map.insert(73, &[1027]);     // 73 => [1027]
            map.insert(26, &[2051]);     // 26 => [2051]
            map.insert(49, &[4099]);     // 49 => [4099]
            map.insert(81, &[8195]);     // 81 => [8195]
            map.insert(97, &[16387]);     // 97 => [16387]
            map.insert(103, &[32771]);     // 103 => [32771]
            map.insert(107, &[65539]);     // 107 => [65539]
            map.insert(115, &[131075]);     // 115 => [131075]
            map.insert(13, &[13]);     // 13 => [13]
            map.insert(21, &[21]);     // 21 => [21]
            map.insert(47, &[133]);     // 47 => [133]
            map.insert(79, &[1029]);     // 79 => [1029]
            map.insert(28, &[2053]);     // 28 => [2053]
            map.insert(55, &[4101]);     // 55 => [4101]
            map.insert(87, &[8197]);     // 87 => [8197]
            map.insert(109, &[65541]);     // 109 => [65541]
            map.insert(117, &[131077]);     // 117 => [131077]
            map.insert(59, &[4105]);     // 59 => [4105]
            map.insert(91, &[8201]);     // 91 => [8201]
            map.insert(121, &[131081]);     // 121 => [131081]
            map.insert(56, &[2081]);     // 56 => [2081]
            map.insert(62, &[2113]);     // 62 => [2113]
            map.insert(88, &[2305]);     // 88 => [2305]
            map.insert(94, &[2561]);     // 94 => [2561]
            map.insert(122, &[18433]);     // 122 => [18433]
            map.insert(124, &[34817]);     // 124 => [34817]
            map.insert(14, &[14]);     // 14 => [14]
            map.insert(22, &[22]);     // 22 => [22]
            map.insert(44, &[134]);     // 44 => [134]
            map.insert(76, &[1030]);     // 76 => [1030]
            map.insert(31, &[2054]);     // 31 => [2054]
            map.insert(52, &[4102]);     // 52 => [4102]
            map.insert(84, &[8198]);     // 84 => [8198]
            map.insert(110, &[65542]);     // 110 => [65542]
            map.insert(118, &[131078]);     // 118 => [131078]
            map.insert(61, &[2114]);     // 61 => [2114]
            map.insert(93, &[2562]);     // 93 => [2562]
            map.insert(127, &[34818]);     // 127 => [34818]
            map.insert(15, &[15]);     // 15 => [15]
            map.insert(23, &[23]);     // 23 => [23]
            map.insert(45, &[135]);     // 45 => [135]
            map.insert(77, &[1031]);     // 77 => [1031]
            map.insert(30, &[2055]);     // 30 => [2055]
            map.insert(53, &[4103]);     // 53 => [4103]
            map.insert(85, &[8199]);     // 85 => [8199]
            map.insert(111, &[65543]);     // 111 => [65543]
            map.insert(119, &[131079]);     // 119 => [131079]
            map.insert(60, &[2115]);     // 60 => [2115]
            map.insert(92, &[2563]);     // 92 => [2563]
            map.insert(126, &[34819]);     // 126 => [34819]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode18_11 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode18_11 {
    fn name(&self) -> String {
        "[18, 11] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        18
    }

    fn dimension(&self) -> usize {
        11
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
        let mut error = BinVector::with_capacity(18);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 18 / 64 + if 18 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(18) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(11);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[18 / 64] & !((1 << 18) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode18_11.generator_matrix();
        assert_eq!(code.ncols(), 18);
        assert_eq!(code.nrows(), 11);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode18_11;
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
            let code = GuavaCode18_11;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, false, true, false, false, true, true, false, false, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, false, true, false, false, true, true, false, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_11;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, true, true, true, false, true, true, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, true, true, true, true, false, true, true, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_11;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, false, true, false, true, true, true, true, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, false, true, false, true, true, true, true, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_11;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, true, true, false, true, true, true, true, false, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, true, true, true, true, true, true, false, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_11;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, false, true, true, true, false, true, true, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, false, true, true, true, false, true, true, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_11;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, true, false, false, false, true, true, false, true, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, true, false, false, false, true, true, false, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_11;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, false, true, false, true, true, true, false, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, false, true, false, false, true, true, false, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_11;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, false, false, true, false, true, false, false, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, false, false, true, false, true, false, false, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_11;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, false, false, true, false, false, false, true, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, false, true, true, false, false, true, false, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_11;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, false, false, false, true, true, true, false, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, false, false, false, true, true, true, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_11;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, true, true, true, true, true, true, true, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, false, true, true, true, true, true, false, true, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_11;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, false, false, true, true, true, false, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, false, false, true, true, true, false, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_11;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, false, false, true, true, true, false, true, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, false, true, true, true, false, true, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_11;
            let randvec = BinVector::from_bools(&[false, false, true, true, false, false, true, false, false, true, false, false, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, true, false, false, true, false, false, false, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_11;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, false, false, false, true, true, true, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, true, true, false, false, true, true, true, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_11;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, true, false, false, true, true, true, true, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, true, false, false, true, true, true, true, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_11;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, true, true, false, true, true, true, true, false, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, true, true, false, true, true, true, true, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_11;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, false, true, true, true, false, true, true, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, true, true, true, false, true, true, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_11;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, true, false, true, false, true, true, true, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, true, false, true, true, true, false, true, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode18_11;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, true, false, false, true, false, true, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, true, false, false, true, false, true, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
