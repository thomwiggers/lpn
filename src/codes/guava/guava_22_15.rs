use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[22, 15]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode22_15;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 1212417 ],
                &[ 2260994 ],
                &[ 2392068 ],
                &[ 2654216 ],
                &[ 3178512 ],
                &[ 458784 ],
                &[ 720960 ],
                &[ 1245312 ],
                &[ 2294016 ],
                &[ 2425344 ],
                &[ 2688000 ],
                &[ 3213312 ],
                &[ 2494464 ],
                &[ 2760704 ],
                &[ 3293184 ],
                
            ], 22));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 2002689 ],
                &[ 2326786 ],
                &[ 3894724 ],
                &[ 533576 ],
                &[ 968592 ],
                &[ 69600 ],
                &[ 4091904 ],
                
            ], 22));
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
            map.insert(44, &[64]);     // 44 => [64]
            map.insert(52, &[128]);     // 52 => [128]
            map.insert(55, &[256]);     // 55 => [256]
            map.insert(49, &[512]);     // 49 => [512]
            map.insert(61, &[1024]);     // 61 => [1024]
            map.insert(37, &[2048]);     // 37 => [2048]
            map.insert(64, &[4096]);     // 64 => [4096]
            map.insert(76, &[8192]);     // 76 => [8192]
            map.insert(84, &[16384]);     // 84 => [16384]
            map.insert(19, &[32768]);     // 19 => [32768]
            map.insert(38, &[65536]);     // 38 => [65536]
            map.insert(87, &[131072]);     // 87 => [131072]
            map.insert(81, &[262144]);     // 81 => [262144]
            map.insert(93, &[524288]);     // 93 => [524288]
            map.insert(69, &[1048576]);     // 69 => [1048576]
            map.insert(70, &[2097152]);     // 70 => [2097152]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(33, &[33]);     // 33 => [33]
            map.insert(45, &[65]);     // 45 => [65]
            map.insert(53, &[129]);     // 53 => [129]
            map.insert(54, &[257]);     // 54 => [257]
            map.insert(48, &[513]);     // 48 => [513]
            map.insert(60, &[1025]);     // 60 => [1025]
            map.insert(36, &[2049]);     // 36 => [2049]
            map.insert(65, &[4097]);     // 65 => [4097]
            map.insert(77, &[8193]);     // 77 => [8193]
            map.insert(85, &[16385]);     // 85 => [16385]
            map.insert(18, &[32769]);     // 18 => [32769]
            map.insert(39, &[65537]);     // 39 => [65537]
            map.insert(86, &[131073]);     // 86 => [131073]
            map.insert(80, &[262145]);     // 80 => [262145]
            map.insert(92, &[524289]);     // 92 => [524289]
            map.insert(68, &[1048577]);     // 68 => [1048577]
            map.insert(71, &[2097153]);     // 71 => [2097153]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(34, &[34]);     // 34 => [34]
            map.insert(46, &[66]);     // 46 => [66]
            map.insert(51, &[514]);     // 51 => [514]
            map.insert(63, &[1026]);     // 63 => [1026]
            map.insert(66, &[4098]);     // 66 => [4098]
            map.insert(78, &[8194]);     // 78 => [8194]
            map.insert(83, &[262146]);     // 83 => [262146]
            map.insert(95, &[524290]);     // 95 => [524290]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(20, &[20]);     // 20 => [20]
            map.insert(40, &[68]);     // 40 => [68]
            map.insert(57, &[1028]);     // 57 => [1028]
            map.insert(72, &[8196]);     // 72 => [8196]
            map.insert(23, &[32772]);     // 23 => [32772]
            map.insert(89, &[524292]);     // 89 => [524292]
            map.insert(24, &[24]);     // 24 => [24]
            map.insert(27, &[32776]);     // 27 => [32776]
            map.insert(29, &[1056]);     // 29 => [1056]
            map.insert(96, &[4128]);     // 96 => [4128]
            map.insert(108, &[8224]);     // 108 => [8224]
            map.insert(116, &[16416]);     // 116 => [16416]
            map.insert(119, &[131104]);     // 119 => [131104]
            map.insert(113, &[262176]);     // 113 => [262176]
            map.insert(125, &[524320]);     // 125 => [524320]
            map.insert(101, &[1048608]);     // 101 => [1048608]
            map.insert(102, &[2097184]);     // 102 => [2097184]
            map.insert(120, &[16448]);     // 120 => [16448]
            map.insert(123, &[131136]);     // 123 => [131136]
            map.insert(105, &[1048640]);     // 105 => [1048640]
            map.insert(106, &[2097216]);     // 106 => [2097216]
            map.insert(99, &[131200]);     // 99 => [131200]
            map.insert(114, &[2097280]);     // 114 => [2097280]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(11, &[11]);     // 11 => [11]
            map.insert(35, &[35]);     // 35 => [35]
            map.insert(47, &[67]);     // 47 => [67]
            map.insert(50, &[515]);     // 50 => [515]
            map.insert(62, &[1027]);     // 62 => [1027]
            map.insert(67, &[4099]);     // 67 => [4099]
            map.insert(79, &[8195]);     // 79 => [8195]
            map.insert(82, &[262147]);     // 82 => [262147]
            map.insert(94, &[524291]);     // 94 => [524291]
            map.insert(13, &[13]);     // 13 => [13]
            map.insert(21, &[21]);     // 21 => [21]
            map.insert(41, &[69]);     // 41 => [69]
            map.insert(56, &[1029]);     // 56 => [1029]
            map.insert(73, &[8197]);     // 73 => [8197]
            map.insert(22, &[32773]);     // 22 => [32773]
            map.insert(88, &[524293]);     // 88 => [524293]
            map.insert(25, &[25]);     // 25 => [25]
            map.insert(26, &[32777]);     // 26 => [32777]
            map.insert(28, &[1057]);     // 28 => [1057]
            map.insert(97, &[4129]);     // 97 => [4129]
            map.insert(109, &[8225]);     // 109 => [8225]
            map.insert(117, &[16417]);     // 117 => [16417]
            map.insert(118, &[131105]);     // 118 => [131105]
            map.insert(112, &[262177]);     // 112 => [262177]
            map.insert(124, &[524321]);     // 124 => [524321]
            map.insert(100, &[1048609]);     // 100 => [1048609]
            map.insert(103, &[2097185]);     // 103 => [2097185]
            map.insert(121, &[16449]);     // 121 => [16449]
            map.insert(122, &[131137]);     // 122 => [131137]
            map.insert(104, &[1048641]);     // 104 => [1048641]
            map.insert(107, &[2097217]);     // 107 => [2097217]
            map.insert(98, &[131201]);     // 98 => [131201]
            map.insert(115, &[2097281]);     // 115 => [2097281]
            map.insert(14, &[14]);     // 14 => [14]
            map.insert(42, &[70]);     // 42 => [70]
            map.insert(59, &[1030]);     // 59 => [1030]
            map.insert(74, &[8198]);     // 74 => [8198]
            map.insert(91, &[524294]);     // 91 => [524294]
            map.insert(31, &[1058]);     // 31 => [1058]
            map.insert(110, &[8226]);     // 110 => [8226]
            map.insert(127, &[524322]);     // 127 => [524322]
            map.insert(15, &[15]);     // 15 => [15]
            map.insert(43, &[71]);     // 43 => [71]
            map.insert(58, &[1031]);     // 58 => [1031]
            map.insert(75, &[8199]);     // 75 => [8199]
            map.insert(90, &[524295]);     // 90 => [524295]
            map.insert(30, &[1059]);     // 30 => [1059]
            map.insert(111, &[8227]);     // 111 => [8227]
            map.insert(126, &[524323]);     // 126 => [524323]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode22_15 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode22_15 {
    fn name(&self) -> String {
        "[22, 15] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        22
    }

    fn dimension(&self) -> usize {
        15
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
        let mut error = BinVector::with_capacity(22);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 22 / 64 + if 22 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(22) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(15);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[22 / 64] & !((1 << 22) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode22_15.generator_matrix();
        assert_eq!(code.ncols(), 22);
        assert_eq!(code.nrows(), 15);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode22_15;
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
            let code = GuavaCode22_15;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, true, true, false, true, true, false, false, true, true, false, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, true, false, true, false, true, true, false, false, true, false, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_15;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, false, true, true, false, false, false, false, true, false, false, false, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, false, true, true, false, false, false, false, true, false, false, true, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_15;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, false, false, true, false, false, false, false, false, true, true, true, true, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, false, false, true, false, false, false, false, false, true, true, true, true, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_15;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, true, true, true, true, false, false, false, false, true, false, false, false, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, true, true, true, true, false, false, false, false, true, false, false, false, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_15;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, true, true, false, false, true, true, false, true, false, true, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, true, true, false, false, true, true, false, true, false, true, true, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_15;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, true, true, true, true, false, false, true, true, true, true, false, true, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, true, true, true, false, false, true, true, true, true, false, true, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_15;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, false, false, true, true, true, true, true, false, true, true, true, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, false, false, true, true, true, true, false, false, true, true, true, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_15;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, false, false, false, true, true, false, false, false, false, false, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, true, true, false, false, true, true, false, false, false, false, false, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_15;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, true, true, false, false, true, false, true, true, true, true, false, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, true, false, false, false, true, false, true, true, true, true, false, true, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_15;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, false, false, true, false, false, false, false, true, false, true, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, false, false, true, false, false, false, false, true, false, true, false, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_15;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, false, true, false, false, false, false, false, true, true, false, false, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, false, true, false, false, false, false, false, true, true, false, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_15;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, false, true, false, true, false, true, false, true, true, false, false, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, true, false, true, false, true, false, true, true, false, false, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_15;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, false, true, false, false, true, true, false, true, true, true, true, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, true, true, false, false, true, true, false, true, false, true, true, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_15;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, false, true, false, false, false, false, false, true, true, false, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, true, true, true, false, false, false, false, false, true, true, false, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_15;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, true, false, false, true, false, true, true, false, true, true, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, true, false, false, true, false, true, true, false, true, true, true, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_15;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, false, true, true, true, true, true, false, false, true, true, true, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, false, true, true, true, true, true, false, true, true, true, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_15;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, true, false, false, false, false, true, false, true, true, false, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, false, true, false, false, false, false, true, false, true, true, false, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_15;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, false, true, true, true, false, false, false, false, false, true, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, false, false, true, true, true, false, false, false, false, false, true, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_15;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, true, true, true, true, true, true, true, true, false, false, true, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, true, true, true, true, true, true, true, false, false, false, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode22_15;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, false, true, false, false, false, true, true, true, true, false, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, true, false, true, false, false, false, true, true, true, true, false, true, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, true, false ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
