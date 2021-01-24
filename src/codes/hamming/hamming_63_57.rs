use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[63, 57]`` Hamming code
///
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct HammingCode63_57;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 6917529027641081857 ],
                &[ 5764607523034234882 ],
                &[ 3458764513820540932 ],
                &[ 5188146770730811400 ],
                &[ 2882303761517117456 ],
                &[ 1729382256910270496 ],
                &[ 8646911284551352384 ],
                &[ 4899916394579099776 ],
                &[ 2594073385365405952 ],
                &[ 1441151880758559232 ],
                &[ 8358680908399641600 ],
                &[ 864691128455137280 ],
                &[ 7782220156096221184 ],
                &[ 6629298651489378304 ],
                &[ 4323455642275692544 ],
                &[ 4755801206503276544 ],
                &[ 2449958197289615360 ],
                &[ 1297036692682833920 ],
                &[ 8214565720324046848 ],
                &[ 720575940379803648 ],
                &[ 7638104968021409792 ],
                &[ 6485183463415611392 ],
                &[ 4179340454204014592 ],
                &[ 432345564235956224 ],
                &[ 7349874591885426688 ],
                &[ 6196953087295356928 ],
                &[ 3891110078115217408 ],
                &[ 5620492335092596736 ],
                &[ 3314649326013120512 ],
                &[ 2161727821674708992 ],
                &[ 9079256849852661760 ],
                &[ 4467570832499015680 ],
                &[ 6773413843860193280 ],
                &[ 7926335352762007552 ],
                &[ 1008806333710860288 ],
                &[ 8502796130835234816 ],
                &[ 1585267137553891328 ],
                &[ 2738188710880215040 ],
                &[ 5044031857532862464 ],
                &[ 8791027022383022080 ],
                &[ 1873498544497754112 ],
                &[ 3026421148616228864 ],
                &[ 5332266356853178368 ],
                &[ 3602888497989419008 ],
                &[ 5908740303296135168 ],
                &[ 7061679400089026560 ],
                &[ 8935212029447241728 ],
                &[ 2017753370550337536 ],
                &[ 3170815612645539840 ],
                &[ 5476940096835944448 ],
                &[ 3748120789879095296 ],
                &[ 6055089698999631872 ],
                &[ 7210263003420164096 ],
                &[ 4044232465378705408 ],
                &[ 6359082673847140352 ],
                &[ 7530018576963469312 ],
                &[ 8142508126285856768 ],
                
            ], 63));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 6794407181578229077 ],
                &[ 7967754386995045990 ],
                &[ 8582797079929714808 ],
                &[ 8926204282575814528 ],
                &[ 9079186482182193152 ],
                &[ 9223372034707292160 ],
                
            ], 63));
            let matrix_t = Box::new(matrix.transposed());
            PARITY_MATRIX = Box::into_raw(matrix);
            PARITY_MATRIX_T = Box::into_raw(matrix_t);

            let mut map = Box::new(FnvHashMap::with_capacity_and_hasher(64, Default::default()));
            map.insert(0, &[0]);     // 0 => [0]
            map.insert(1, &[1]);     // 1 => [1]
            map.insert(2, &[2]);     // 2 => [2]
            map.insert(3, &[4]);     // 3 => [4]
            map.insert(4, &[8]);     // 4 => [8]
            map.insert(5, &[16]);     // 5 => [16]
            map.insert(6, &[32]);     // 6 => [32]
            map.insert(7, &[64]);     // 7 => [64]
            map.insert(8, &[128]);     // 8 => [128]
            map.insert(9, &[256]);     // 9 => [256]
            map.insert(10, &[512]);     // 10 => [512]
            map.insert(11, &[1024]);     // 11 => [1024]
            map.insert(12, &[2048]);     // 12 => [2048]
            map.insert(13, &[4096]);     // 13 => [4096]
            map.insert(14, &[8192]);     // 14 => [8192]
            map.insert(15, &[16384]);     // 15 => [16384]
            map.insert(16, &[32768]);     // 16 => [32768]
            map.insert(17, &[65536]);     // 17 => [65536]
            map.insert(18, &[131072]);     // 18 => [131072]
            map.insert(19, &[262144]);     // 19 => [262144]
            map.insert(20, &[524288]);     // 20 => [524288]
            map.insert(21, &[1048576]);     // 21 => [1048576]
            map.insert(22, &[2097152]);     // 22 => [2097152]
            map.insert(23, &[4194304]);     // 23 => [4194304]
            map.insert(24, &[8388608]);     // 24 => [8388608]
            map.insert(25, &[16777216]);     // 25 => [16777216]
            map.insert(26, &[33554432]);     // 26 => [33554432]
            map.insert(27, &[67108864]);     // 27 => [67108864]
            map.insert(28, &[134217728]);     // 28 => [134217728]
            map.insert(29, &[268435456]);     // 29 => [268435456]
            map.insert(30, &[536870912]);     // 30 => [536870912]
            map.insert(31, &[1073741824]);     // 31 => [1073741824]
            map.insert(32, &[2147483648]);     // 32 => [2147483648]
            map.insert(33, &[4294967296]);     // 33 => [4294967296]
            map.insert(34, &[8589934592]);     // 34 => [8589934592]
            map.insert(35, &[17179869184]);     // 35 => [17179869184]
            map.insert(36, &[34359738368]);     // 36 => [34359738368]
            map.insert(37, &[68719476736]);     // 37 => [68719476736]
            map.insert(38, &[137438953472]);     // 38 => [137438953472]
            map.insert(39, &[274877906944]);     // 39 => [274877906944]
            map.insert(40, &[549755813888]);     // 40 => [549755813888]
            map.insert(41, &[1099511627776]);     // 41 => [1099511627776]
            map.insert(42, &[2199023255552]);     // 42 => [2199023255552]
            map.insert(43, &[4398046511104]);     // 43 => [4398046511104]
            map.insert(44, &[8796093022208]);     // 44 => [8796093022208]
            map.insert(45, &[17592186044416]);     // 45 => [17592186044416]
            map.insert(46, &[35184372088832]);     // 46 => [35184372088832]
            map.insert(48, &[70368744177664]);     // 48 => [70368744177664]
            map.insert(49, &[140737488355328]);     // 49 => [140737488355328]
            map.insert(50, &[281474976710656]);     // 50 => [281474976710656]
            map.insert(51, &[562949953421312]);     // 51 => [562949953421312]
            map.insert(52, &[1125899906842624]);     // 52 => [1125899906842624]
            map.insert(53, &[2251799813685248]);     // 53 => [2251799813685248]
            map.insert(54, &[4503599627370496]);     // 54 => [4503599627370496]
            map.insert(56, &[9007199254740992]);     // 56 => [9007199254740992]
            map.insert(57, &[18014398509481984]);     // 57 => [18014398509481984]
            map.insert(58, &[36028797018963968]);     // 58 => [36028797018963968]
            map.insert(60, &[72057594037927936]);     // 60 => [72057594037927936]
            map.insert(47, &[144115188075855872]);     // 47 => [144115188075855872]
            map.insert(55, &[288230376151711744]);     // 55 => [288230376151711744]
            map.insert(59, &[576460752303423488]);     // 59 => [576460752303423488]
            map.insert(61, &[1152921504606846976]);     // 61 => [1152921504606846976]
            map.insert(62, &[2305843009213693952]);     // 62 => [2305843009213693952]
            map.insert(63, &[4611686018427387904]);     // 63 => [4611686018427387904]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl HammingCode63_57 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for HammingCode63_57 {
    fn name(&self) -> String {
        "[63, 57] Hamming code".to_owned()
    }

    fn length(&self) -> usize {
        63
    }

    fn dimension(&self) -> usize {
        57
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
        let mut error = BinVector::with_capacity(63);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 63 / 64 + if 63 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(63) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(57);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[63 / 64] & !((1 << 63) - 1), 0, "this message has excess bits");

        let map = unsafe {
            SYNDROME_MAP.as_ref().unwrap()
        };
        let he = &BinMatrix::from_slices(&[&c[..]], self.length()) * self.parity_check_matrix_transposed();
        let error = map[unsafe { &he.get_word_unchecked(0, 0) }];
        c.iter_mut().zip(error.iter().copied()).for_each(|(sample, error)| *sample ^= error as u64);
    }

    
    /// We know how to give the bias directly for this code
    fn bias(&self, delta: f64) -> f64 {
        
        (1f64 + f64::from(63) * delta) / f64::from(63 + 1)
        
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use m4ri_rust::friendly::BinVector;
    use crate::oracle::Sample;

    #[test]
    fn size() {
        let code = HammingCode63_57.generator_matrix();
        assert_eq!(code.ncols(), 63);
        assert_eq!(code.nrows(), 57);
    }

    #[test]
    fn test_decode_sample() {
        let code = HammingCode63_57;
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
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, true, false, false, true, true, true, false, true, false, true, false, true, true, true, false, false, false, true, true, true, false, false, true, true, true, false, false, true, true, false, true, true, true, true, false, true, false, true, false, false, false, true, false, true, false, true, false, true, true, false, false, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, true, true, false, false, true, true, true, false, true, false, true, false, true, true, true, false, false, false, true, true, true, false, false, true, true, true, false, false, true, true, false, true, true, true, true, false, true, false, false, false, false, false, true, false, true, false, true, false, true, true, false, false, true, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, false, false, true, true, false, true, true, false, true, false, true, true, false, true, false, false, false, true, true, false, false, true, true, true, true, true, false, false, true, true, true, true, true, false, false, true, true, true, false, false, false, false, true, true, true, false, true, true, true, false, false, false, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, true, false, false, true, true, true, true, true, false, true, false, true, true, false, true, false, false, false, true, true, false, false, true, true, true, true, true, false, false, true, true, true, true, true, false, false, true, true, true, false, false, false, false, true, true, true, false, true, true, true, false, false, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, false, true, true, true, false, true, true, false, true, true, true, true, false, false, false, false, true, true, false, true, false, false, false, true, false, false, true, false, true, false, false, true, true, false, false, false, false, true, false, true, true, false, false, false, true, false, false, true, false, false, false, false, true, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, false, true, true, true, false, true, true, true, true, true, true, true, false, false, false, false, true, true, false, true, false, false, false, true, false, false, true, false, true, false, false, true, true, false, false, false, false, true, false, true, true, false, false, false, true, false, false, true, false, false, false, false, true, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, true, false, true, false, true, true, false, false, false, false, true, false, false, false, true, true, false, true, false, false, true, true, true, true, false, true, true, true, false, true, false, false, true, true, false, true, false, false, true, false, true, true, false, true, false, false, false, true, false, true, false, true, true, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, true, false, true, false, true, true, false, false, false, false, true, false, false, false, true, true, false, true, false, false, true, true, true, true, false, true, true, true, false, true, false, false, true, true, false, true, false, false, true, false, true, true, false, true, false, false, false, true, false, true, false, true, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, true, true, true, false, true, true, true, true, false, false, true, false, false, true, false, true, false, false, false, false, false, true, false, false, false, true, false, true, false, true, true, true, true, true, false, true, false, true, true, true, true, false, true, true, true, false, false, true, false, false, false, true, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, false, true, true, true, false, true, true, true, true, false, false, true, false, false, true, false, true, false, false, false, false, false, true, false, false, false, true, false, true, false, true, true, true, true, true, false, true, false, true, true, true, true, false, true, true, true, false, false, true, false, false, false, true, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, true, false, true, false, false, true, false, true, false, false, true, true, false, true, true, true, false, true, true, true, false, false, false, true, false, false, false, true, true, false, true, true, true, false, false, true, false, false, false, false, true, false, true, true, true, true, true, true, false, false, false, true, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, true, true, false, true, false, false, true, false, true, false, false, true, true, false, true, true, true, false, true, true, true, false, false, false, true, false, false, false, true, true, false, true, true, true, false, false, true, false, false, false, false, true, false, true, false, true, true, true, true, false, false, false, true, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, true, false, true, false, false, true, false, true, false, false, true, true, false, false, false, true, false, true, true, true, false, true, false, false, true, true, true, false, false, true, false, false, true, false, false, true, true, false, false, true, true, false, false, false, true, false, false, false, false, false, false, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, true, true, false, true, false, false, true, false, true, false, false, true, true, false, false, false, true, false, true, true, true, false, true, true, false, true, true, true, false, false, true, false, false, true, false, false, true, true, false, false, true, true, false, false, false, true, false, false, false, false, false, false, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, false, false, false, false, true, false, true, false, false, true, true, false, true, true, true, true, false, true, false, true, false, false, false, false, true, true, true, false, false, false, false, false, false, false, true, false, false, true, true, false, false, false, true, false, true, false, true, false, false, false, true, true, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, false, false, false, false, true, false, true, false, false, true, true, false, true, true, true, true, false, true, false, true, false, false, false, false, true, true, true, false, false, false, false, false, false, false, true, false, false, true, true, false, true, false, true, false, true, false, true, false, false, false, true, true, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, false, false, false, true, true, true, true, true, false, false, true, false, false, false, false, true, false, false, true, false, true, true, true, true, false, true, true, false, true, true, true, true, false, true, false, false, false, false, true, false, false, true, true, true, false, false, true, false, false, false, false, false, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, false, false, true, true, true, true, true, false, false, true, false, false, false, false, true, true, false, true, false, true, true, true, true, false, true, true, false, true, true, true, true, false, true, false, false, false, false, true, false, false, true, true, true, false, false, true, false, false, false, false, false, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, true, true, false, false, false, false, true, true, true, true, false, false, false, true, true, false, false, true, false, false, false, true, false, false, false, false, false, true, true, true, true, true, true, true, false, true, false, false, true, true, false, true, true, true, true, false, false, false, false, false, false, false, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, true, false, false, true, false, true, true, true, true, false, false, false, true, true, false, false, true, false, false, false, true, false, false, false, false, false, true, true, true, true, true, true, true, false, true, false, false, true, true, false, true, true, true, true, false, false, false, false, false, false, false, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, false, false, true, true, true, false, false, true, true, true, true, true, true, false, true, true, false, false, false, false, true, true, true, false, true, false, true, false, false, false, true, false, true, false, true, false, true, true, false, true, true, true, false, true, true, false, false, true, true, true, true, false, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, false, true, true, false, false, false, true, true, true, true, true, true, false, true, true, false, false, false, false, true, true, true, false, true, false, true, false, false, false, true, false, true, false, true, false, true, true, false, true, true, true, false, true, true, false, false, true, true, true, true, false, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, true, true, false, true, false, false, false, true, false, false, true, false, true, false, true, false, false, true, true, true, true, false, false, false, true, false, false, false, false, false, true, false, false, false, true, false, false, true, false, true, true, false, true, true, true, false, true, true, false, true, false, false, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, true, true, false, true, false, false, false, true, false, false, true, false, true, false, true, false, false, true, true, true, true, false, false, false, true, false, false, false, false, false, true, false, false, false, true, false, false, true, false, true, true, false, true, true, true, false, true, true, false, true, false, false, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, false, false, false, true, false, false, false, false, false, true, false, true, true, true, false, false, true, true, false, true, false, true, false, false, true, true, true, false, true, true, true, false, true, false, false, false, true, false, true, false, true, true, false, false, false, false, true, false, true, true, true, false, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, false, false, false, true, false, false, false, false, false, true, false, true, true, true, false, false, true, true, false, true, false, true, false, false, true, true, true, false, true, true, true, false, true, false, false, false, true, false, true, false, true, true, false, false, false, false, true, false, true, true, true, false, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, true, false, true, true, false, false, true, true, false, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, false, true, true, true, true, false, false, false, true, false, false, true, false, false, false, false, false, true, true, true, false, false, true, true, false, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, true, false, true, true, false, false, true, true, false, false, true, false, true, false, true, false, true, false, true, false, true, true, true, false, true, false, false, true, true, true, true, false, false, false, true, false, false, true, false, false, false, false, false, true, true, true, false, false, true, true, false, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, true, false, true, false, false, true, true, true, true, false, false, false, true, false, true, true, true, false, false, true, true, false, false, true, false, true, true, true, false, false, false, false, true, true, false, false, false, true, false, false, false, true, false, true, false, false, true, false, false, false, false, true, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, false, true, false, false, true, true, true, true, false, false, false, true, false, true, true, true, false, false, true, true, false, false, true, false, true, true, true, false, false, false, false, true, true, false, false, false, true, false, false, false, true, false, true, false, false, true, false, false, true, false, true, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, true, true, true, true, false, true, true, true, true, true, false, false, false, true, true, false, false, true, false, true, false, true, true, false, true, false, false, true, false, false, false, true, true, false, true, true, false, false, false, false, true, false, true, false, false, false, false, false, false, true, true, true, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, true, true, true, false, false, true, true, true, true, true, false, false, false, true, true, false, false, true, false, true, false, true, true, false, true, false, false, true, false, false, false, true, true, false, true, true, false, false, false, false, true, false, true, false, false, false, false, false, false, true, true, true, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, false, true, false, false, true, false, true, false, true, false, false, true, false, false, false, true, true, false, false, false, true, false, true, true, true, true, false, true, true, false, true, true, false, true, true, true, false, false, false, false, false, true, false, true, false, false, false, false, true, true, true, true, false, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, false, true, false, false, true, false, true, false, true, false, false, true, false, false, false, true, true, false, true, false, true, false, true, true, true, true, false, true, true, false, true, true, false, true, true, true, false, false, false, false, false, true, false, true, false, false, false, false, true, true, true, true, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, true, true, true, true, false, false, false, false, false, false, true, false, false, true, true, true, true, true, false, false, false, true, true, true, true, true, false, false, true, true, false, false, true, false, true, true, false, false, false, true, true, false, false, false, false, false, false, false, false, true, true, true, false, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, true, true, true, true, false, false, false, false, false, false, true, false, false, true, true, true, true, true, false, false, false, true, true, true, true, true, false, false, true, true, false, false, true, false, true, true, false, false, false, true, true, false, false, false, false, false, false, false, false, true, true, true, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, false, false, true, false, false, false, true, true, false, false, true, false, false, true, false, false, true, false, false, false, true, true, false, false, false, true, true, true, false, true, true, true, false, false, false, false, false, false, true, false, false, true, false, false, false, false, true, false, true, false, false, true, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, false, false, true, false, false, false, true, true, false, false, true, false, false, true, false, false, true, false, false, false, true, true, false, false, false, true, true, true, false, false, true, true, false, false, false, false, false, false, true, false, false, true, false, false, false, false, true, false, true, false, false, true, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode63_57;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, false, true, false, true, false, true, false, false, false, false, false, false, false, true, false, true, true, true, false, false, false, true, true, true, true, false, false, false, true, false, false, true, true, false, false, true, false, true, true, true, false, false, true, true, true, true, true, true, true, false, false, false, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, false, true, false, true, false, true, false, false, false, false, false, false, false, true, false, true, true, true, false, false, false, true, true, true, true, false, false, false, true, false, false, true, true, false, false, true, false, true, false, true, false, false, true, true, true, true, true, true, true, false, false, false, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
