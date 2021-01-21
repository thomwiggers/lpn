use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[23, 16]`` Guava code
///
/// Best code found from the GUAVA database version 3.15
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct GuavaCode23_16;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 1376257 ],
                &[ 2424834 ],
                &[ 4521988 ],
                &[ 4784136 ],
                &[ 5308432 ],
                &[ 6357024 ],
                &[ 917568 ],
                &[ 1441920 ],
                &[ 2490624 ],
                &[ 4588032 ],
                &[ 4850688 ],
                &[ 5376000 ],
                &[ 6426624 ],
                &[ 4988928 ],
                &[ 5521408 ],
                &[ 6586368 ],
                
            ], 23));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 1937185 ],
                &[ 2134306 ],
                &[ 4653572 ],
                &[ 7789448 ],
                &[ 903088 ],
                &[ 139200 ],
                &[ 8183808 ],
                
            ], 23));
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
            map.insert(59, &[256]);     // 59 => [256]
            map.insert(61, &[512]);     // 61 => [512]
            map.insert(49, &[1024]);     // 49 => [1024]
            map.insert(41, &[2048]);     // 41 => [2048]
            map.insert(42, &[4096]);     // 42 => [4096]
            map.insert(64, &[8192]);     // 64 => [8192]
            map.insert(88, &[16384]);     // 88 => [16384]
            map.insert(91, &[32768]);     // 91 => [32768]
            map.insert(21, &[65536]);     // 21 => [65536]
            map.insert(44, &[131072]);     // 44 => [131072]
            map.insert(93, &[262144]);     // 93 => [262144]
            map.insert(81, &[524288]);     // 81 => [524288]
            map.insert(73, &[1048576]);     // 73 => [1048576]
            map.insert(74, &[2097152]);     // 74 => [2097152]
            map.insert(76, &[4194304]);     // 76 => [4194304]
            map.insert(3, &[3]);     // 3 => [3]
            map.insert(5, &[5]);     // 5 => [5]
            map.insert(9, &[9]);     // 9 => [9]
            map.insert(17, &[17]);     // 17 => [17]
            map.insert(18, &[33]);     // 18 => [33]
            map.insert(33, &[65]);     // 33 => [65]
            map.insert(57, &[129]);     // 57 => [129]
            map.insert(58, &[257]);     // 58 => [257]
            map.insert(60, &[513]);     // 60 => [513]
            map.insert(48, &[1025]);     // 48 => [1025]
            map.insert(40, &[2049]);     // 40 => [2049]
            map.insert(43, &[4097]);     // 43 => [4097]
            map.insert(65, &[8193]);     // 65 => [8193]
            map.insert(89, &[16385]);     // 89 => [16385]
            map.insert(90, &[32769]);     // 90 => [32769]
            map.insert(20, &[65537]);     // 20 => [65537]
            map.insert(45, &[131073]);     // 45 => [131073]
            map.insert(92, &[262145]);     // 92 => [262145]
            map.insert(80, &[524289]);     // 80 => [524289]
            map.insert(72, &[1048577]);     // 72 => [1048577]
            map.insert(75, &[2097153]);     // 75 => [2097153]
            map.insert(77, &[4194305]);     // 77 => [4194305]
            map.insert(6, &[6]);     // 6 => [6]
            map.insert(10, &[10]);     // 10 => [10]
            map.insert(34, &[66]);     // 34 => [66]
            map.insert(63, &[514]);     // 63 => [514]
            map.insert(51, &[1026]);     // 51 => [1026]
            map.insert(66, &[8194]);     // 66 => [8194]
            map.insert(23, &[65538]);     // 23 => [65538]
            map.insert(46, &[131074]);     // 46 => [131074]
            map.insert(95, &[262146]);     // 95 => [262146]
            map.insert(83, &[524290]);     // 83 => [524290]
            map.insert(78, &[4194306]);     // 78 => [4194306]
            map.insert(12, &[12]);     // 12 => [12]
            map.insert(36, &[68]);     // 36 => [68]
            map.insert(53, &[1028]);     // 53 => [1028]
            map.insert(68, &[8196]);     // 68 => [8196]
            map.insert(85, &[524292]);     // 85 => [524292]
            map.insert(24, &[24]);     // 24 => [24]
            map.insert(27, &[40]);     // 27 => [40]
            map.insert(29, &[65544]);     // 29 => [65544]
            map.insert(96, &[8256]);     // 96 => [8256]
            map.insert(120, &[16448]);     // 120 => [16448]
            map.insert(123, &[32832]);     // 123 => [32832]
            map.insert(125, &[262208]);     // 125 => [262208]
            map.insert(113, &[524352]);     // 113 => [524352]
            map.insert(105, &[1048640]);     // 105 => [1048640]
            map.insert(106, &[2097216]);     // 106 => [2097216]
            map.insert(108, &[4194368]);     // 108 => [4194368]
            map.insert(99, &[32896]);     // 99 => [32896]
            map.insert(101, &[262272]);     // 101 => [262272]
            map.insert(114, &[2097280]);     // 114 => [2097280]
            map.insert(116, &[4194432]);     // 116 => [4194432]
            map.insert(102, &[262400]);     // 102 => [262400]
            map.insert(119, &[4194560]);     // 119 => [4194560]
            map.insert(7, &[7]);     // 7 => [7]
            map.insert(11, &[11]);     // 11 => [11]
            map.insert(35, &[67]);     // 35 => [67]
            map.insert(62, &[515]);     // 62 => [515]
            map.insert(50, &[1027]);     // 50 => [1027]
            map.insert(67, &[8195]);     // 67 => [8195]
            map.insert(22, &[65539]);     // 22 => [65539]
            map.insert(47, &[131075]);     // 47 => [131075]
            map.insert(94, &[262147]);     // 94 => [262147]
            map.insert(82, &[524291]);     // 82 => [524291]
            map.insert(79, &[4194307]);     // 79 => [4194307]
            map.insert(13, &[13]);     // 13 => [13]
            map.insert(37, &[69]);     // 37 => [69]
            map.insert(52, &[1029]);     // 52 => [1029]
            map.insert(69, &[8197]);     // 69 => [8197]
            map.insert(84, &[524293]);     // 84 => [524293]
            map.insert(25, &[25]);     // 25 => [25]
            map.insert(26, &[41]);     // 26 => [41]
            map.insert(28, &[65545]);     // 28 => [65545]
            map.insert(97, &[8257]);     // 97 => [8257]
            map.insert(121, &[16449]);     // 121 => [16449]
            map.insert(122, &[32833]);     // 122 => [32833]
            map.insert(124, &[262209]);     // 124 => [262209]
            map.insert(112, &[524353]);     // 112 => [524353]
            map.insert(104, &[1048641]);     // 104 => [1048641]
            map.insert(107, &[2097217]);     // 107 => [2097217]
            map.insert(109, &[4194369]);     // 109 => [4194369]
            map.insert(98, &[32897]);     // 98 => [32897]
            map.insert(100, &[262273]);     // 100 => [262273]
            map.insert(115, &[2097281]);     // 115 => [2097281]
            map.insert(117, &[4194433]);     // 117 => [4194433]
            map.insert(103, &[262401]);     // 103 => [262401]
            map.insert(118, &[4194561]);     // 118 => [4194561]
            map.insert(14, &[14]);     // 14 => [14]
            map.insert(38, &[70]);     // 38 => [70]
            map.insert(55, &[1030]);     // 55 => [1030]
            map.insert(70, &[8198]);     // 70 => [8198]
            map.insert(87, &[524294]);     // 87 => [524294]
            map.insert(31, &[65546]);     // 31 => [65546]
            map.insert(127, &[262210]);     // 127 => [262210]
            map.insert(110, &[4194370]);     // 110 => [4194370]
            map.insert(15, &[15]);     // 15 => [15]
            map.insert(39, &[71]);     // 39 => [71]
            map.insert(54, &[1031]);     // 54 => [1031]
            map.insert(71, &[8199]);     // 71 => [8199]
            map.insert(86, &[524295]);     // 86 => [524295]
            map.insert(30, &[65547]);     // 30 => [65547]
            map.insert(126, &[262211]);     // 126 => [262211]
            map.insert(111, &[4194371]);     // 111 => [4194371]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl GuavaCode23_16 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for GuavaCode23_16 {
    fn name(&self) -> String {
        "[23, 16] Guava code".to_owned()
    }

    fn length(&self) -> usize {
        23
    }

    fn dimension(&self) -> usize {
        16
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
        let mut error = BinVector::with_capacity(23);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 23 / 64 + if 23 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(23) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(16);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[23 / 64] & !((1 << 23) - 1), 0, "this message has excess bits");

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
        let code = GuavaCode23_16.generator_matrix();
        assert_eq!(code.ncols(), 23);
        assert_eq!(code.nrows(), 16);
    }

    #[test]
    fn test_decode_sample() {
        let code = GuavaCode23_16;
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
            let code = GuavaCode23_16;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, true, false, false, true, false, false, false, true, false, true, false, true, true, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, true, true, false, true, false, false, false, true, false, true, true, true, true, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_16;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, true, true, false, false, false, false, true, true, false, false, false, false, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, true, true, false, false, false, false, false, true, true, false, false, false, false, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_16;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, false, true, false, true, true, true, false, true, false, false, false, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, true, true, false, true, true, true, false, true, false, false, false, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_16;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, false, false, true, false, false, true, true, true, true, true, true, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, true, false, false, false, false, false, true, true, true, true, true, true, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_16;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, false, false, false, true, true, false, true, false, false, false, true, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, false, true, false, false, true, true, false, true, false, false, true, true, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_16;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, true, false, false, false, true, false, false, true, true, true, true, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, false, false, false, false, true, false, false, true, true, true, true, true, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_16;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, false, false, false, false, true, true, true, false, true, false, false, true, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, false, false, false, false, true, true, true, false, true, false, true, true, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_16;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, true, true, false, false, false, false, true, true, true, false, false, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, true, false, false, false, false, false, true, true, true, false, false, false, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_16;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, true, false, true, false, false, true, false, false, true, true, true, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, true, false, true, false, false, true, true, false, true, true, true, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_16;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, true, true, true, true, false, true, false, false, true, true, false, true, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, false, true, true, true, true, false, true, false, false, true, true, false, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_16;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, false, false, true, false, false, false, true, false, false, true, false, true, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, true, false, true, false, false, false, true, false, false, true, false, true, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_16;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, true, true, true, false, false, true, false, true, false, true, false, true, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, false, false, true, true, false, false, true, false, true, false, true, false, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_16;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, true, false, true, false, true, false, true, false, false, false, true, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, false, false, true, false, true, false, true, false, false, false, true, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_16;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, false, true, false, true, true, false, true, false, false, false, false, true, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, false, true, false, true, true, false, true, false, false, false, false, false, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_16;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, false, false, true, true, true, false, false, false, true, false, true, false, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, true, false, false, false, true, true, false, false, false, true, false, true, false, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_16;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, true, true, false, false, true, false, true, true, false, true, false, false, false, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, true, false, false, true, false, true, true, true, true, false, false, false, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_16;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, true, true, true, false, false, true, false, true, false, true, true, true, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, false, false, true, true, false, false, true, false, true, false, true, true, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_16;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, true, false, false, true, false, true, true, false, false, true, false, false, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, false, true, false, false, true, false, true, true, false, false, true, false, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_16;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, false, false, false, false, false, false, true, true, true, false, true, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, false, false, false, false, false, false, true, true, true, true, true, true, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = GuavaCode23_16;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, true, false, true, false, false, true, false, false, false, false, true, false, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, true, false, false, true, false, false, true, false, false, false, false, true, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, true, false, false ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
