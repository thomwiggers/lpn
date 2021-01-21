use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[24, 17]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode24_17;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 1703937 ],
                &[ 2752514 ],
                &[ 4849668 ],
                &[ 9043976 ],
                &[ 9568272 ],
                &[ 10616864 ],
                &[ 12714048 ],
                &[ 1835136 ],
                &[ 2883840 ],
                &[ 4981248 ],
                &[ 9176064 ],
                &[ 9701376 ],
                &[ 10752000 ],
                &[ 12853248 ],
                &[ 9977856 ],
                &[ 11042816 ],
                &[ 13172736 ],
                
            ], 24));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 1806177 ],
                &[ 2134306 ],
                &[ 4268612 ],
                &[ 9307144 ],
                &[ 16136304 ],
                &[ 278400 ],
                &[ 16367616 ],
                
            ], 24));
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
            map.insert(41, &[1024]);     // 41 => [1024]
            map.insert(49, &[2048]);     // 49 => [2048]
            map.insert(50, &[4096]);     // 50 => [4096]
            map.insert(52, &[8192]);     // 52 => [8192]
            map.insert(64, &[16384]);     // 64 => [16384]
            map.insert(67, &[32768]);     // 67 => [32768]
            map.insert(69, &[65536]);     // 69 => [65536]
            map.insert(25, &[131072]);     // 25 => [131072]
            map.insert(56, &[262144]);     // 56 => [262144]
            map.insert(73, &[524288]);     // 73 => [524288]
            map.insert(81, &[1048576]);     // 81 => [1048576]
            map.insert(82, &[2097152]);     // 82 => [2097152]
            map.insert(84, &[4194304]);     // 84 => [4194304]
            map.insert(88, &[8388608]);     // 88 => [8388608]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(18, &[33]);     // 18 => [33]
            map.insert(20, &[65]);     // 20 => [65]
            map.insert(33, &[129]);     // 33 => [129]
            map.insert(34, &[257]);     // 34 => [257]
            map.insert(36, &[513]);     // 36 => [513]
            map.insert(40, &[1025]);     // 40 => [1025]
            map.insert(48, &[2049]);     // 48 => [2049]
            map.insert(51, &[4097]);     // 51 => [4097]
            map.insert(53, &[8193]);     // 53 => [8193]
            map.insert(65, &[16385]);     // 65 => [16385]
            map.insert(66, &[32769]);     // 66 => [32769]
            map.insert(68, &[65537]);     // 68 => [65537]
            map.insert(24, &[131073]);     // 24 => [131073]
            map.insert(57, &[262145]);     // 57 => [262145]
            map.insert(72, &[524289]);     // 72 => [524289]
            map.insert(80, &[1048577]);     // 80 => [1048577]
            map.insert(83, &[2097153]);     // 83 => [2097153]
            map.insert(85, &[4194305]);     // 85 => [4194305]
            map.insert(89, &[8388609]);     // 89 => [8388609]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(23, &[66]);     // 23 => [66]
            map.insert(39, &[514]);     // 39 => [514]
            map.insert(43, &[1026]);     // 43 => [1026]
            map.insert(54, &[8194]);     // 54 => [8194]
            map.insert(71, &[65538]);     // 71 => [65538]
            map.insert(27, &[131074]);     // 27 => [131074]
            map.insert(58, &[262146]);     // 58 => [262146]
            map.insert(75, &[524290]);     // 75 => [524290]
            map.insert(86, &[4194306]);     // 86 => [4194306]
            map.insert(90, &[8388610]);     // 90 => [8388610]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(45, &[1028]);     // 45 => [1028]
            map.insert(29, &[131076]);     // 29 => [131076]
            map.insert(60, &[262148]);     // 60 => [262148]
            map.insert(77, &[524292]);     // 77 => [524292]
            map.insert(92, &[8388612]);     // 92 => [8388612]
            map.insert(96, &[16512]);     // 96 => [16512]
            map.insert(99, &[32896]);     // 99 => [32896]
            map.insert(101, &[65664]);     // 101 => [65664]
            map.insert(105, &[524416]);     // 105 => [524416]
            map.insert(113, &[1048704]);     // 113 => [1048704]
            map.insert(114, &[2097280]);     // 114 => [2097280]
            map.insert(116, &[4194432]);     // 116 => [4194432]
            map.insert(120, &[8388736]);     // 120 => [8388736]
            map.insert(102, &[65792]);     // 102 => [65792]
            map.insert(106, &[524544]);     // 106 => [524544]
            map.insert(119, &[4194560]);     // 119 => [4194560]
            map.insert(123, &[8388864]);     // 123 => [8388864]
            map.insert(108, &[524800]);     // 108 => [524800]
            map.insert(125, &[8389120]);     // 125 => [8389120]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(11, &[11]);     // 11 => [11]
            map.insert(22, &[67]);     // 22 => [67]
            map.insert(38, &[515]);     // 38 => [515]
            map.insert(42, &[1027]);     // 42 => [1027]
            map.insert(55, &[8195]);     // 55 => [8195]
            map.insert(70, &[65539]);     // 70 => [65539]
            map.insert(26, &[131075]);     // 26 => [131075]
            map.insert(59, &[262147]);     // 59 => [262147]
            map.insert(74, &[524291]);     // 74 => [524291]
            map.insert(87, &[4194307]);     // 87 => [4194307]
            map.insert(91, &[8388611]);     // 91 => [8388611]
            map.insert(13, &[13]);     // 13 => [13]
            map.insert(44, &[1029]);     // 44 => [1029]
            map.insert(28, &[131077]);     // 28 => [131077]
            map.insert(61, &[262149]);     // 61 => [262149]
            map.insert(76, &[524293]);     // 76 => [524293]
            map.insert(93, &[8388613]);     // 93 => [8388613]
            map.insert(97, &[16513]);     // 97 => [16513]
            map.insert(98, &[32897]);     // 98 => [32897]
            map.insert(100, &[65665]);     // 100 => [65665]
            map.insert(104, &[524417]);     // 104 => [524417]
            map.insert(112, &[1048705]);     // 112 => [1048705]
            map.insert(115, &[2097281]);     // 115 => [2097281]
            map.insert(117, &[4194433]);     // 117 => [4194433]
            map.insert(121, &[8388737]);     // 121 => [8388737]
            map.insert(103, &[65793]);     // 103 => [65793]
            map.insert(107, &[524545]);     // 107 => [524545]
            map.insert(118, &[4194561]);     // 118 => [4194561]
            map.insert(122, &[8388865]);     // 122 => [8388865]
            map.insert(109, &[524801]);     // 109 => [524801]
            map.insert(124, &[8389121]);     // 124 => [8389121]
            map.insert(14, &[14]);     // 14 => [14]
            map.insert(47, &[1030]);     // 47 => [1030]
            map.insert(31, &[131078]);     // 31 => [131078]
            map.insert(62, &[262150]);     // 62 => [262150]
            map.insert(79, &[524294]);     // 79 => [524294]
            map.insert(94, &[8388614]);     // 94 => [8388614]
            map.insert(110, &[524802]);     // 110 => [524802]
            map.insert(127, &[8389122]);     // 127 => [8389122]
            map.insert(15, &[15]);     // 15 => [15]
            map.insert(46, &[1031]);     // 46 => [1031]
            map.insert(30, &[131079]);     // 30 => [131079]
            map.insert(63, &[262151]);     // 63 => [262151]
            map.insert(78, &[524295]);     // 78 => [524295]
            map.insert(95, &[8388615]);     // 95 => [8388615]
            map.insert(111, &[524803]);     // 111 => [524803]
            map.insert(126, &[8389123]);     // 126 => [8389123]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode24_17 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode24_17 {
    fn name(&self) -> String {
        "[24, 17] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        24
    }

    fn dimension(&self) -> usize {
        17
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
        let mut error = BinVector::with_capacity(24);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 24 / 64 + if 24 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(24) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(17);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[24 / 64] & !((1 << 24) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode24_17.generator_matrix();
        assert_eq!(code.ncols(), 24);
        assert_eq!(code.nrows(), 17);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode24_17;
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
            let code = GuavaCode24_17;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, true, false, true, false, false, true, true, true, true, true, false, true, false, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, false, false, true, false, false, true, true, true, true, true, false, true, false, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_17;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, true, false, true, true, false, true, false, false, false, true, false, true, false, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, true, false, true, false, false, true, false, false, false, true, false, true, false, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_17;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, true, true, true, false, false, false, false, true, true, true, false, false, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, true, true, true, true, true, false, false, false, true, true, true, false, false, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_17;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, false, true, true, true, false, true, false, true, true, false, false, false, true, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, true, true, true, true, true, false, true, true, false, false, false, true, false, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_17;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, true, true, true, false, true, true, false, true, true, false, false, false, true, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, true, true, true, false, true, true, false, true, true, false, false, false, true, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_17;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, true, false, true, true, true, false, false, true, true, true, false, false, true, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, false, true, true, true, true, true, false, false, true, true, true, false, false, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_17;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, true, true, false, false, false, true, false, false, false, true, true, false, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, true, true, false, false, false, true, false, false, false, true, true, false, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_17;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, true, false, false, false, true, false, true, true, true, false, true, true, true, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, false, true, false, false, false, true, false, true, true, true, false, true, true, true, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_17;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, false, true, false, true, true, false, false, true, true, true, true, false, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, false, true, false, true, true, false, false, true, true, true, true, false, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_17;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, false, true, false, false, false, false, false, true, false, false, true, false, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, false, true, false, false, false, false, false, true, false, false, true, true, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_17;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, false, false, false, true, true, true, true, false, true, true, false, false, true, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, false, false, false, true, true, true, false, false, true, true, false, false, true, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_17;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, false, true, true, true, true, true, true, false, false, false, true, false, true, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, true, true, true, true, true, true, false, false, true, true, false, true, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_17;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, false, true, true, false, true, false, true, true, false, true, false, false, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, false, false, true, true, false, true, false, true, true, false, true, false, false, true, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_17;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, true, true, false, false, true, false, false, false, false, true, false, true, false, true, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, true, true, false, false, true, false, false, false, false, true, true, true, false, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_17;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, false, true, false, true, true, true, false, false, true, false, false, false, true, true, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, true, false, true, true, true, false, false, true, false, false, false, true, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_17;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, false, false, true, false, false, false, true, true, true, true, true, true, true, true, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, false, false, true, false, true, false, true, true, true, true, true, true, true, true, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_17;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, true, true, true, false, false, false, true, false, true, false, false, true, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, true, true, true, false, false, false, true, false, true, false, false, true, true, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_17;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, false, false, false, true, true, true, true, false, false, false, false, true, true, true, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, false, false, true, true, true, true, true, false, false, false, false, true, true, true, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_17;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, true, false, true, false, true, false, true, false, true, false, true, false, false, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, false, true, true, true, false, true, false, true, false, true, false, false, true, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode24_17;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, false, false, false, true, false, false, true, true, true, false, true, false, true, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, false, false, false, true, false, false, true, true, true, false, true, false, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, true, false, false, false ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
