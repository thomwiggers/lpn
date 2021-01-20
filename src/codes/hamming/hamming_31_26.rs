use std::boxed::Box;
use std::default::Default;
use std::sync::Once;

use fnv::FnvHashMap;

use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use crate::codes::BinaryCode;

/// ``[31, 26]`` Hamming code
///
/// Decodes using Syndrome decoding
#[derive(Clone, Serialize)]
pub struct HammingCode31_26;

static INIT: Once = Once::new();
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX_T: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<u64, &'static [usize; 1]> = 0 as *const FnvHashMap<u64, &'static [usize; 1]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 1610612737 ],
                &[ 1342177282 ],
                &[ 805306372 ],
                &[ 1207959560 ],
                &[ 671088656 ],
                &[ 402653216 ],
                &[ 2013265984 ],
                &[ 1140850816 ],
                &[ 603980032 ],
                &[ 335544832 ],
                &[ 1946158080 ],
                &[ 201328640 ],
                &[ 1811943424 ],
                &[ 1543512064 ],
                &[ 1006649344 ],
                &[ 2080407552 ],
                &[ 469827584 ],
                &[ 738328576 ],
                &[ 1275330560 ],
                &[ 872939520 ],
                &[ 1410334720 ],
                &[ 1679818752 ],
                &[ 943718400 ],
                &[ 1484783616 ],
                &[ 1761607680 ],
                &[ 1912602624 ],
                
            ], 31));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::from_slices(&[
                &[ 1553290581 ],
                &[ 1831233126 ],
                &[ 1983412344 ],
                &[ 2076213120 ],
                &[ 2147450880 ],
                
            ], 31));
            let matrix_t = Box::new(matrix.transposed());
            PARITY_MATRIX = Box::into_raw(matrix);
            PARITY_MATRIX_T = Box::into_raw(matrix_t);

            let mut map = Box::new(FnvHashMap::with_capacity_and_hasher(32, Default::default()));
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
            map.insert(23, &[67108864]);     // 23 => [67108864]
            map.insert(24, &[4194304]);     // 24 => [4194304]
            map.insert(25, &[8388608]);     // 25 => [8388608]
            map.insert(26, &[16777216]);     // 26 => [16777216]
            map.insert(27, &[134217728]);     // 27 => [134217728]
            map.insert(28, &[33554432]);     // 28 => [33554432]
            map.insert(29, &[268435456]);     // 29 => [268435456]
            map.insert(30, &[536870912]);     // 30 => [536870912]
            map.insert(31, &[1073741824]);     // 31 => [1073741824]
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}

impl HammingCode31_26 {
    fn parity_check_matrix_transposed(&self) -> &BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX_T.as_ref().unwrap()
        }
    }
}

impl BinaryCode for HammingCode31_26 {
    fn name(&self) -> String {
        "[31, 26] Hamming code".to_owned()
    }

    fn length(&self) -> usize {
        31
    }

    fn dimension(&self) -> usize {
        26
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
        let mut error = BinVector::with_capacity(31);
        let stor = unsafe { error.get_storage_mut() };
        let errbytes = map[&he.as_u64()];
        debug_assert_eq!(errbytes.len(), 31 / 64 + if 31 % 64 != 0 { 1 } else { 0 });
        stor.clear();
        stor.extend_from_slice(&errbytes[..]);
        unsafe { error.set_len(31) };
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((&result * self.parity_check_matrix_transposed()).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(26);
        Ok(codeword)
        
    }

    fn decode_slice(&self, c: &mut [u64]) {
        init();
        
        debug_assert_eq!(c[31 / 64] & !((1 << 31) - 1), 0, "this message has excess bits");

        let map = unsafe {
            SYNDROME_MAP.as_ref().unwrap()
        };
        let he = &BinMatrix::from_slices(&[&c[..]], self.length()) * self.parity_check_matrix_transposed();
        let error = map[unsafe { &he.get_word_unchecked(0, 0) }];
        c.iter_mut().zip(error.iter().copied()).for_each(|(sample, error)| *sample ^= error as u64);
    }

    
    /// We know how to give the bias directly for this code
    fn bias(&self, delta: f64) -> f64 {
        
        (1f64 + f64::from(31) * delta) / f64::from(31 + 1)
        
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use m4ri_rust::friendly::BinVector;
    use crate::oracle::Sample;

    #[test]
    fn size() {
        let code = HammingCode31_26.generator_matrix();
        assert_eq!(code.ncols(), 31);
        assert_eq!(code.nrows(), 26);
    }

    #[test]
    fn test_decode_sample() {
        let code = HammingCode31_26;
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
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, true, true, true, false, true, true, true, false, false, false, true, false, false, false, false, false, true, false, true, true, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, false, true, true, true, false, true, true, true, false, false, false, true, false, false, false, false, false, true, false, true, true, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, false, true, true, true, false, false, false, false, true, false, false, false, false, true, true, true, true, true, true, true, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, true, false, true, true, true, false, false, true, false, true, false, false, false, false, true, true, true, true, true, true, true, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, true, false, true, false, false, true, true, false, true, true, false, true, false, true, false, true, false, false, true, true, false, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, true, false, true, false, false, true, false, false, true, true, false, true, false, true, false, true, false, false, true, true, false, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, false, true, false, false, false, true, false, false, false, true, false, false, false, true, true, true, false, true, false, true, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, false, true, false, false, false, true, false, false, false, true, false, false, false, true, true, true, false, true, true, true, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, true, false, true, true, false, true, true, true, false, false, true, true, true, false, false, true, false, true, false, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, false, true, false, true, true, false, true, true, true, false, true, true, true, true, false, false, true, false, true, false, true, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, true, true, true, false, false, false, true, false, true, false, true, false, true, true, true, false, false, true, true, true, true, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, true, true, false, false, false, true, false, true, false, true, false, true, true, true, false, false, true, true, true, true, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, false, false, true, true, false, false, true, false, true, true, true, true, true, false, false, true, false, false, false, true, true, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, false, false, true, true, false, false, true, false, true, true, true, true, true, false, false, true, false, false, false, true, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, false, false, false, true, true, true, false, true, true, true, true, true, false, false, true, true, true, true, false, false, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, false, false, false, true, true, true, false, true, true, true, true, true, false, false, true, false, true, true, false, false, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, true, false, true, false, true, true, false, false, false, false, true, true, false, true, false, false, true, true, false, false, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, true, true, false, false, false, true, true, false, false, false, false, true, true, false, true, false, false, true, true, false, false, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, true, false, false, true, false, true, false, false, true, false, true, false, true, false, false, true, true, true, true, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, true, false, false, true, false, true, false, false, true, false, true, false, true, false, false, true, true, true, true, true, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, false, false, false, true, false, false, true, false, true, true, true, true, true, false, true, true, false, true, true, false, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, false, false, true, false, false, true, false, true, true, true, true, true, false, true, true, false, true, false, false, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, true, true, false, false, true, true, false, false, false, true, true, true, false, true, true, true, true, true, false, true, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, false, true, true, false, false, true, true, false, false, false, true, true, true, false, true, true, true, true, true, false, true, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, true, true, true, true, false, false, false, false, false, false, true, true, true, false, true, true, false, true, true, true, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, false, false, true, true, true, false, false, false, false, false, false, true, true, true, false, true, true, false, true, true, true, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, false, false, false, true, true, true, false, true, false, true, true, false, true, true, true, false, true, false, true, false, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, true, false, false, false, true, true, true, false, true, false, true, true, false, true, true, true, false, true, false, true, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, true, false, true, false, true, false, true, true, true, true, false, true, false, true, false, true, false, true, false, false, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, true, false, true, false, true, false, true, true, true, true, false, true, false, true, false, true, false, false, false, false, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, true, false, true, false, false, true, false, true, false, true, false, true, false, false, true, true, true, false, true, true, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, false, true, false, false, true, false, true, false, true, false, true, false, false, true, true, true, false, true, true, true, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, true, false, false, true, false, false, false, false, false, false, true, true, false, true, false, true, true, false, false, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, true, false, false, true, false, false, false, false, false, false, true, false, false, true, false, true, true, false, false, true, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, true, false, true, true, true, true, true, false, true, true, false, true, false, true, true, true, false, true, true, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, false, true, false, true, true, true, true, true, false, true, false, false, true, false, true, true, true, false, true, true, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, true, false, false, true, true, true, true, false, true, true, false, true, false, true, true, false, false, false, false, false, true, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, false, false, true, true, true, true, false, true, true, false, true, false, true, true, true, false, false, false, false, true, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, true, true, true, false, false, false, true, true, false, false, false, true, true, false, false, false, true, true, false, true, false, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, true, true, true, false, false, false, true, true, false, false, false, true, true, false, false, false, true, true, false, true, false, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, false, true, true, false, false, false, false, true, false, false, true, false, false, true, true, true, false, true, true, false, false, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, false, true, true, false, false, false, false, true, false, false, true, false, false, true, true, true, false, true, true, false, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, false, false, false, false, true, true, false, true, false, true, false, false, false, true, false, true, true, true, false, false, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, false, false, false, false, true, true, false, true, false, true, false, false, false, true, false, true, false, true, false, false, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, false, true, false, true, false, false, false, false, false, true, false, false, false, false, true, true, true, true, false, true, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, true, false, true, false, false, false, false, false, true, false, false, false, false, false, true, true, true, false, true, true, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, false, false, false, false, false, false, false, false, false, true, true, false, true, false, false, false, true, true, false, false, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, false, false, false, false, false, false, false, false, true, true, true, true, false, false, false, true, true, false, false, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, false, true, false, false, false, true, true, true, false, false, true, false, false, true, false, true, true, true, false, false, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, false, false, true, false, false, false, false, true, true, false, false, true, false, false, true, false, true, true, true, false, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, true, true, true, true, true, true, false, false, false, true, true, true, true, true, false, true, false, true, false, true, true, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, true, true, true, true, true, false, false, false, true, false, true, true, true, false, true, false, true, false, true, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, false, false, true, true, true, true, true, false, true, false, false, true, false, true, true, true, false, false, true, true, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, false, false, true, true, true, false, true, false, true, false, false, true, false, true, true, true, false, false, true, true, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, false, true, true, false, false, true, true, true, false, false, true, false, false, false, true, false, true, false, true, false, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, false, false, true, true, false, true, true, true, true, false, false, true, false, false, false, true, false, true, false, true, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, true, true, true, true, true, false, false, true, true, false, true, true, true, true, true, true, true, false, false, false, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, true, true, true, true, true, false, true, true, true, false, true, true, true, true, true, true, true, false, false, false, true, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, true, true, true, false, false, true, false, true, false, true, false, true, true, false, true, false, true, true, false, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, true, true, true, true, false, true, true, false, true, false, true, false, true, true, false, true, false, true, true, false, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, true, false, false, true, false, true, true, false, true, true, true, false, false, false, true, false, false, true, false, true, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, true, false, false, true, false, true, true, false, true, true, true, false, false, false, true, false, false, true, false, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, false, false, true, false, true, true, true, true, false, false, true, true, true, true, true, true, false, false, true, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, false, false, true, false, true, true, true, true, false, false, true, true, true, true, true, true, false, false, true, false, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, false, true, false, true, false, false, true, false, true, false, true, false, true, true, true, false, true, false, false, true, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, false, true, false, true, false, false, true, false, true, false, true, false, true, true, true, false, true, false, true, true, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, true, true, true, false, false, true, false, false, true, true, true, false, false, true, true, true, false, false, false, false, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, false, true, true, false, false, true, false, false, true, true, true, false, false, true, true, true, false, false, false, false, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, false, false, true, true, false, true, true, true, true, false, true, true, true, true, true, false, false, false, true, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, false, false, true, true, false, false, true, true, true, false, true, true, true, true, true, false, false, false, true, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, false, false, false, false, true, false, false, false, true, true, true, false, false, false, false, true, true, true, true, true, true, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, false, false, false, false, true, false, false, false, false, true, true, false, false, false, false, true, true, true, true, true, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, true, false, false, true, true, false, true, false, true, true, false, true, true, true, false, false, false, true, false, false, false, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, false, false, true, true, false, false, false, true, true, false, true, true, true, false, false, false, true, false, false, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, false, true, true, true, false, false, true, false, true, false, true, true, true, true, true, false, true, false, false, false, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, false, false, true, true, true, false, false, true, false, true, false, true, true, false, true, true, false, true, false, false, false, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, true, false, false, false, false, false, false, true, true, true, true, true, true, true, false, false, true, false, true, true, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, true, false, false, true, false, false, false, true, true, true, true, true, true, true, false, false, true, false, true, true, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, true, true, true, false, true, false, true, false, false, true, false, false, false, false, true, true, true, true, true, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, false, true, true, true, false, true, false, true, false, false, false, false, false, false, false, true, true, true, true, true, true, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, true, true, true, false, true, false, false, false, true, true, false, true, false, false, false, false, true, true, true, false, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, true, true, false, true, false, false, false, true, true, false, true, false, false, false, false, true, false, true, false, false, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, true, false, false, true, true, false, true, false, true, true, true, false, false, true, false, false, false, false, false, true, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, true, false, false, true, true, false, true, false, true, true, true, false, false, true, false, false, false, false, false, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, false, false, true, true, false, true, true, false, false, true, true, false, false, false, false, true, false, false, false, false, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, false, false, true, true, false, true, true, false, false, true, true, false, false, false, false, true, false, false, false, false, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, false, true, false, false, false, true, false, true, false, true, true, true, true, false, true, true, true, true, true, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, false, false, true, false, false, false, true, false, true, false, true, true, true, true, true, true, true, true, true, true, true, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, true, true, false, true, false, false, false, false, true, true, false, true, false, false, true, false, false, false, false, true, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, true, true, true, false, true, false, false, false, false, true, true, true, true, false, false, true, false, false, false, false, true, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, true, false, false, false, true, false, true, true, true, true, true, false, false, false, true, false, false, true, true, true, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, true, false, false, false, true, false, true, true, true, true, true, false, false, false, true, false, false, true, true, true, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, true, false, true, true, true, false, true, false, false, false, false, true, false, true, false, false, false, true, false, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, false, true, false, true, true, true, false, true, false, false, false, false, true, true, true, false, false, false, true, false, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, true, true, false, false, false, false, false, true, true, true, false, false, true, false, false, true, true, true, true, true, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, true, true, false, false, false, false, false, true, true, true, false, false, true, false, false, true, true, true, true, true, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, false, true, true, true, true, true, false, true, true, true, false, true, false, true, false, false, true, false, true, true, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, false, true, true, true, false, true, false, true, true, true, false, true, false, true, false, false, true, false, true, true, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, true, false, true, false, true, false, true, true, false, true, true, true, true, false, false, true, true, false, false, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, true, false, true, false, false, false, true, true, false, true, true, true, true, false, false, true, true, false, false, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, false, true, false, false, true, true, true, true, false, false, true, true, true, true, true, false, true, true, false, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, false, false, true, false, false, true, true, true, true, false, false, true, true, true, true, true, false, true, true, false, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, true, false, false, true, false, true, false, false, false, false, false, true, false, false, true, false, true, false, true, false, true, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, true, false, false, true, false, true, false, false, false, true, false, true, false, false, true, false, true, false, true, false, true, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, true, true, true, false, false, true, true, false, false, true, false, true, true, true, true, true, true, false, true, true, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, true, true, true, false, false, true, true, false, false, true, false, true, true, true, true, true, true, false, true, true, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, false, false, true, false, true, false, false, false, true, true, false, false, true, false, true, true, true, false, true, true, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, false, false, false, false, false, true, false, false, false, true, true, false, false, true, false, true, true, true, false, true, true, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, true, true, false, false, true, true, false, false, false, false, true, true, false, true, true, true, false, true, false, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, true, true, false, false, true, true, false, false, false, false, true, true, true, true, true, true, false, true, false, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, true, false, false, true, true, false, true, true, true, true, false, true, false, true, false, false, false, true, true, false, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, false, false, true, true, false, true, true, true, true, false, true, false, false, false, false, false, true, true, false, true, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, true, false, true, false, false, true, false, false, false, false, false, true, false, false, true, false, true, true, false, true, false, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, false, true, false, false, true, false, false, false, false, false, true, false, false, true, false, true, true, false, true, false, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, true, true, false, false, false, false, false, true, true, false, false, false, true, false, true, true, true, true, false, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, true, true, false, false, true, false, false, true, true, false, false, false, true, false, true, true, true, true, false, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, false, true, false, false, false, false, true, false, false, true, false, true, false, true, true, false, false, false, true, true, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, false, true, false, false, false, false, true, false, false, true, false, true, false, true, true, false, false, false, true, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, false, true, true, false, false, true, false, true, true, false, true, true, true, true, false, false, true, false, true, true, false, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, false, true, true, false, true, true, false, true, true, false, true, true, true, true, false, false, true, false, true, true, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, true, false, true, false, false, false, false, true, false, false, true, true, false, false, true, false, false, false, false, true, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, true, false, true, false, false, false, false, true, false, false, true, true, false, false, false, false, false, false, false, true, false, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, false, false, false, false, true, false, false, true, true, false, true, false, true, true, false, true, false, false, false, false, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, true, false, false, false, false, true, false, false, true, true, false, true, false, true, true, false, true, false, false, false, false, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, true, true, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, true, false, true, true, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, true, false, false, false, false, false, false, true, false, false, true, false, false, false, false, false, true, false, true, true, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, true, false, true, true, true, true, true, true, true, true, false, false, false, false, true, true, false, false, false, true, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, true, false, true, true, true, true, true, true, true, true, false, false, false, false, true, true, false, false, false, true, false, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, false, true, true, false, true, false, false, false, false, false, true, true, true, false, true, true, false, false, true, true, false, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, true, false, false, true, false, false, false, false, false, true, true, true, false, true, true, false, false, true, true, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, true, false, true, true, true, false, true, false, true, false, false, true, true, false, true, false, true, false, true, false, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, true, true, false, true, true, true, false, true, false, true, false, false, true, true, false, true, false, true, false, true, false, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, true, false, true, true, true, true, false, true, true, true, true, true, true, true, true, false, true, false, false, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, true, false, true, true, true, true, false, true, true, true, true, true, true, true, true, false, true, false, false, false, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, true, false, true, false, false, false, false, true, false, false, true, false, false, true, true, true, true, false, false, false, true, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, true, false, true, false, false, false, false, true, false, false, true, false, false, true, true, true, true, false, true, false, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, true, true, true, false, false, false, true, false, false, false, false, false, false, true, true, false, false, true, true, false, true, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, true, true, true, false, false, false, false, false, false, false, false, false, false, true, true, false, false, true, true, false, true, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, false, false, false, false, false, true, true, false, false, true, true, true, true, true, false, false, false, false, false, false, false, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, true, false, false, false, false, true, true, false, false, true, true, true, true, true, false, false, false, false, false, false, false, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, false, false, true, false, false, true, false, true, true, true, true, false, false, true, true, false, true, true, true, true, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, true, false, false, true, false, false, false, false, true, true, true, true, false, false, true, true, false, true, true, true, true, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, false, false, false, true, true, true, true, false, false, false, false, false, true, false, true, true, true, false, false, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, false, false, false, false, true, true, true, true, true, false, false, false, false, true, false, true, true, true, false, false, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, true, false, true, true, false, false, false, false, false, true, false, false, false, true, false, true, false, false, true, false, true, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, false, true, true, false, false, false, false, false, true, false, false, false, true, false, true, false, false, true, false, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, false, false, true, false, false, false, false, true, false, false, true, false, false, true, true, false, true, false, true, false, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, false, false, true, false, false, false, false, true, false, false, true, false, false, true, true, false, true, true, true, false, true, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, true, false, false, true, false, false, false, false, false, true, true, true, false, true, true, false, false, false, false, true, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, true, false, true, true, false, false, false, false, false, true, true, true, false, true, true, false, false, false, false, true, true, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, true, false, false, true, false, true, false, true, true, false, true, false, false, false, true, false, true, false, false, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, true, false, false, true, false, true, false, true, true, false, true, false, false, false, true, false, false, false, false, true, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, false, false, false, true, true, true, true, true, false, false, true, true, false, false, false, false, false, false, false, false, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, false, false, false, true, true, true, true, true, false, false, true, true, false, false, false, false, false, false, false, false, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, false, true, true, true, true, false, false, true, false, true, false, true, true, false, false, false, true, true, true, true, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, false, true, true, true, false, false, false, true, false, true, false, true, true, false, false, false, true, true, true, true, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, true, true, true, true, true, false, true, false, true, false, false, false, true, true, true, false, false, false, true, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, true, true, true, true, false, false, true, false, true, false, false, false, true, true, true, false, false, false, true, true, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, false, true, false, true, true, true, true, true, false, true, true, false, false, false, false, false, false, true, false, false, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, false, true, false, true, true, true, true, true, false, true, true, false, false, false, false, false, false, true, false, false, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, false, true, false, true, false, false, false, true, true, false, true, false, false, true, true, true, false, false, false, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, true, false, true, false, true, false, false, false, true, true, false, true, false, false, true, true, false, false, false, false, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, false, true, false, false, true, false, true, true, false, true, false, false, true, false, false, false, false, false, false, true, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, false, true, false, false, true, false, true, true, false, false, false, false, true, false, false, false, false, false, false, true, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, false, false, true, true, false, false, false, false, false, false, true, true, false, true, false, false, true, true, true, false, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, false, false, true, true, false, false, false, false, false, false, true, true, false, true, false, false, true, true, true, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, false, true, true, false, false, false, false, true, false, false, true, true, false, false, true, true, true, true, false, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, false, true, true, true, false, false, false, true, false, false, true, true, false, false, true, true, true, true, false, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, false, true, false, false, false, false, false, true, true, true, true, false, false, false, false, true, false, true, false, true, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, true, false, false, false, false, false, true, true, true, true, false, false, false, false, true, false, true, false, true, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, false, false, true, false, true, false, false, false, true, true, false, true, false, false, false, false, false, false, true, true, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, false, true, false, true, false, false, false, true, true, false, true, true, false, false, false, false, false, true, true, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, false, false, true, true, true, true, true, true, true, true, false, true, true, false, false, false, false, false, true, true, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, false, false, true, true, true, true, true, true, true, true, false, true, true, false, true, false, false, false, true, true, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, true, true, true, true, true, true, true, false, true, true, false, true, true, true, false, false, false, false, true, false, false, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, true, true, true, true, true, true, false, false, true, true, false, true, true, true, false, false, false, false, true, false, false, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, true, false, false, true, false, false, false, false, false, false, true, false, true, false, true, true, false, true, false, false, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, true, false, false, true, false, false, false, false, false, false, true, false, true, true, true, true, false, true, false, false, true, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, true, false, false, false, true, false, true, true, false, false, true, false, true, true, false, false, true, false, true, false, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, false, true, false, false, false, false, false, true, true, false, false, true, false, true, true, false, false, true, false, true, false, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, true, true, false, false, false, true, true, true, false, true, false, true, true, true, true, true, false, false, true, false, true, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, true, true, false, false, false, true, true, true, false, true, false, true, true, true, true, false, false, false, true, false, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, false, false, false, false, true, false, false, true, false, false, false, true, false, true, true, true, true, true, true, false, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, true, false, false, false, false, true, true, false, true, false, false, false, true, false, true, true, true, true, true, true, false, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, false, true, true, true, false, true, true, false, true, true, false, true, false, true, false, true, true, true, true, true, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, false, true, true, true, false, true, true, false, true, true, false, true, false, true, false, true, false, true, true, true, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, true, true, true, false, true, false, false, true, true, false, true, true, true, true, false, false, false, false, true, false, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, true, true, true, false, true, false, false, true, true, false, true, false, true, true, false, false, false, false, true, false, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, false, false, false, true, true, false, false, false, false, false, false, true, true, false, true, true, false, false, true, false, true, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, false, false, true, false, false, false, false, false, false, false, true, true, false, true, true, false, false, true, false, true, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, false, true, true, false, false, true, true, true, false, true, true, true, false, false, true, false, false, false, false, false, true, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, true, true, false, false, true, true, false, false, true, true, true, false, false, true, false, false, false, false, false, true, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, false, false, true, false, false, true, false, true, false, false, true, false, false, false, false, false, true, false, true, true, true, false, false, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, false, false, true, false, true, false, false, true, false, false, false, false, false, true, false, true, true, true, false, false, false, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, true, false, true, true, false, false, true, false, true, false, true, true, true, true, false, false, true, false, false, false, true, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, true, false, false, true, false, false, true, false, true, false, true, true, true, true, false, false, true, false, false, false, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, false, false, true, true, false, true, true, false, true, true, false, false, true, true, false, false, false, false, false, false, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, true, false, false, true, true, false, true, true, false, true, true, false, false, true, true, false, false, false, false, false, false, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, true, true, false, false, true, true, true, true, false, true, false, true, true, true, true, true, true, false, false, false, false, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, true, true, false, false, true, true, false, true, false, true, false, true, true, true, true, true, true, false, false, false, false, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, false, false, false, true, false, true, true, true, true, true, false, true, false, false, true, true, false, true, true, true, false, true, false, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, false, true, true, true, true, true, false, false, false, false, true, true, false, true, true, true, false, true, false, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, false, true, true, true, false, true, false, false, true, true, false, true, true, false, true, false, true, true, true, true, false, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, true, true, true, false, true, false, false, true, false, false, true, true, false, true, false, true, true, true, true, false, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, true, true, true, true, true, false, false, true, false, true, false, true, true, true, true, false, true, true, false, true, true, false, false, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, true, true, true, false, false, true, false, true, false, true, true, true, true, false, true, true, false, true, true, false, false, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, false, true, true, true, true, false, true, true, true, false, true, false, false, true, false, true, false, true, true, false, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, false, true, true, true, true, false, true, true, true, false, true, false, false, false, false, true, false, true, true, false, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, false, false, false, true, true, true, true, true, false, true, true, true, false, false, true, false, false, false, true, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, true, false, false, false, true, true, true, true, true, false, true, true, true, false, false, true, false, false, false, true, false, true, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, false, true, false, false, false, false, true, true, true, true, true, false, true, false, false, true, false, true, true, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, true, false, true, false, true, false, false, true, true, true, true, true, false, true, false, false, true, false, true, true, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, false, true, false, false, true, true, false, true, false, false, true, true, true, false, false, true, false, true, false, false, true, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, true, false, false, false, true, false, true, false, false, true, true, true, false, false, true, false, true, false, false, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, true, false, true, true, false, true, false, false, true, false, true, false, false, false, true, false, false, false, false, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, true, false, true, true, false, true, false, false, true, false, true, false, false, false, true, false, false, false, false, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, false, true, false, true, false, true, false, false, true, false, false, true, true, true, true, false, true, false, false, true, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, true, false, true, false, true, false, false, true, false, true, true, true, true, true, false, true, false, false, true, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, false, false, true, false, false, true, false, false, false, false, true, false, true, false, false, true, false, true, false, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, false, false, false, true, false, false, true, false, false, false, false, true, false, false, false, false, true, false, true, false, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, false, false, true, false, false, false, false, false, false, true, true, true, false, true, false, true, false, false, true, true, true, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, false, false, true, false, false, false, false, false, false, true, true, true, false, true, true, true, false, false, true, true, true, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, true, false, false, true, false, true, true, true, true, false, true, true, true, false, true, false, false, false, false, false, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, true, false, false, true, false, true, false, true, true, false, true, true, true, false, true, false, false, false, false, false, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, true, true, false, false, true, false, false, false, false, true, false, false, true, true, true, false, true, true, true, false, true, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, true, true, false, false, true, false, false, false, false, true, false, false, true, true, true, false, true, true, true, false, false, false, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, true, true, true, true, true, true, true, true, true, false, false, true, false, false, false, false, true, true, false, false, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, true, true, false, true, true, true, true, true, false, false, true, false, false, false, false, true, true, false, false, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, false, true, false, false, true, true, true, true, true, true, true, true, false, false, true, true, false, false, true, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, false, false, true, false, false, true, true, true, false, true, true, true, true, false, false, true, true, false, false, true, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, false, true, true, false, false, false, true, false, false, true, false, false, true, true, false, true, true, false, false, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, false, false, true, true, false, false, false, true, false, false, true, false, false, true, true, false, false, true, false, false, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, true, true, true, false, true, true, false, false, true, false, false, false, true, true, true, false, true, false, false, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, true, true, true, false, true, true, false, false, false, false, false, false, true, true, true, false, true, false, false, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, true, false, false, false, false, true, true, false, true, true, false, false, false, false, false, false, false, true, true, true, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, false, false, false, false, true, true, false, true, true, false, false, false, false, false, false, false, true, true, true, true, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, true, false, true, true, true, true, false, true, true, true, true, false, false, true, false, false, true, true, false, true, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, false, true, false, true, true, true, true, false, true, true, true, true, false, false, true, false, false, true, true, false, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, true, false, false, true, false, false, true, false, false, false, false, true, true, true, false, false, true, false, true, false, true, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, false, false, true, false, false, true, false, false, false, false, true, true, true, false, false, true, false, true, false, true, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, false, false, false, true, false, false, true, true, false, true, false, true, false, true, true, true, true, true, true, true, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, false, false, false, true, false, true, true, true, false, true, false, true, false, true, true, true, true, true, true, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, true, false, false, true, false, true, true, false, false, false, true, true, false, false, true, true, true, true, true, true, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, true, false, false, true, false, true, true, false, false, false, true, true, false, false, true, true, true, true, true, true, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, false, false, true, true, false, true, false, true, false, false, true, false, false, false, true, false, false, false, false, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, false, false, true, true, false, true, false, true, false, false, true, false, false, false, true, false, false, false, false, true, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, false, true, true, false, true, true, true, true, true, true, false, false, false, false, false, false, false, true, true, true, false, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, true, true, false, true, true, true, true, true, true, false, false, false, false, false, false, false, true, true, true, false, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, false, false, true, true, false, true, true, true, false, false, true, false, true, false, true, false, true, true, false, false, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, false, false, false, true, true, false, true, true, true, false, false, true, false, true, false, true, false, true, false, false, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, true, false, true, true, true, true, false, true, true, false, false, false, false, false, false, true, true, false, true, true, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, true, false, true, true, true, true, false, true, true, false, false, false, true, false, false, true, true, false, true, true, false, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, true, false, true, false, false, false, true, false, true, false, false, false, true, false, true, false, false, true, false, false, true, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, true, false, true, false, false, false, true, false, true, false, false, false, true, false, true, false, false, true, false, false, true, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, true, true, false, false, false, true, true, false, false, true, true, false, true, false, false, false, true, true, true, true, false, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, true, true, false, false, false, true, true, false, false, true, true, false, true, false, false, false, true, true, true, true, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, true, false, false, true, false, true, false, true, false, false, false, true, true, false, false, true, true, false, false, true, true, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, true, false, false, true, false, true, false, true, false, false, false, true, true, false, false, true, false, false, false, true, true, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, true, true, false, false, true, false, false, true, false, true, false, true, false, false, true, true, false, true, true, false, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, true, false, false, true, false, false, true, false, true, false, true, false, false, true, true, false, true, true, false, false, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, false, false, false, false, true, false, true, true, false, false, true, false, false, true, true, true, true, true, false, false, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, false, false, false, false, false, true, false, true, true, false, false, true, false, false, true, true, true, true, true, false, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, true, false, true, true, true, false, false, true, true, true, true, false, false, false, true, false, false, true, true, false, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, true, true, false, true, true, true, false, false, true, true, true, true, false, false, false, true, false, false, true, true, false, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, true, false, false, false, true, true, false, false, false, true, true, false, false, false, true, true, false, true, false, true, false, true, false, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, true, false, false, false, true, false, false, false, false, true, true, false, false, false, true, true, false, true, false, true, false, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, true, false, false, false, false, false, false, true, true, false, false, true, true, true, false, true, true, false, true, false, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, false, true, false, false, false, false, false, false, true, true, false, false, true, true, true, false, true, false, false, true, false, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, false, true, false, false, true, true, false, false, false, false, false, true, false, false, false, true, false, false, false, false, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, false, false, true, false, false, true, true, false, false, false, false, false, true, false, false, false, true, false, false, false, false, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, true, true, false, true, true, true, true, true, false, false, true, false, true, true, false, false, true, true, true, true, true, false, true, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, true, false, true, true, true, true, true, false, false, true, false, true, true, false, false, true, true, true, true, true, false, true, false, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, true, true, true, false, true, false, true, true, false, false, true, true, true, false, true, false, false, false, false, false, true, false, true, true, true, true, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true, true, true, false, true, false, true, false, false, false, true, true, true, false, true, false, false, false, false, false, true, false, true, true, true, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, true, true, false, true, true, true, false, false, true, false, true, false, true, true, true, false, false, false, false, false, false, false, false, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, true, true, true, false, false, true, false, true, false, true, true, true, false, false, false, false, false, false, false, false, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, false, true, true, true, false, false, false, true, true, false, false, true, false, false, false, true, false, true, false, true, true, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, false, true, true, true, false, false, false, true, true, false, false, true, false, false, false, true, true, true, false, true, true, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, false, false, true, false, true, false, true, true, true, false, true, false, false, false, true, false, false, false, false, true, false, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, false, false, true, false, true, false, true, true, true, false, true, false, false, false, true, false, true, false, false, true, false, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, false, true, false, true, false, false, false, false, true, false, true, false, true, true, true, false, true, false, true, false, true, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, true, false, true, false, false, false, false, true, false, true, false, true, true, false, false, true, false, true, false, true, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, true, false, false, true, true, true, false, false, false, true, false, false, false, false, true, true, true, false, true, true, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, true, false, false, true, true, true, false, false, false, true, true, false, false, false, true, true, true, false, true, true, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, true, false, true, true, true, false, true, false, false, false, false, true, true, true, true, true, false, true, true, true, false, true, false, false, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, true, true, true, false, true, false, false, false, false, true, true, true, true, true, false, true, true, true, false, true, true, false, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, false, false, true, false, true, true, true, false, true, true, false, false, false, true, false, true, false, false, true, true, true, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, false, false, true, false, true, true, true, false, true, true, false, false, false, true, false, true, false, false, true, true, true, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, true, true, false, true, true, false, true, true, false, true, true, true, false, true, true, false, true, false, true, true, true, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, true, true, false, true, true, false, true, true, false, true, true, true, false, false, true, false, true, false, true, true, true, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, false, false, true, true, true, true, true, true, true, false, false, true, true, false, false, false, false, false, false, true, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, false, true, true, true, true, true, true, true, false, false, true, true, false, false, false, false, false, false, true, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, false, false, false, false, false, false, true, true, false, false, false, false, false, false, true, false, true, false, false, true, true, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, false, false, false, false, false, false, true, true, false, false, false, false, false, false, true, false, true, false, false, true, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, false, true, false, true, false, true, false, false, false, false, false, false, false, false, false, false, true, false, true, true, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, false, true, false, true, false, true, false, false, false, false, true, false, false, false, false, false, true, false, true, true, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, true, true, true, true, false, false, false, true, true, true, false, false, false, false, true, false, false, true, false, false, false, true, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, true, true, true, false, false, false, false, true, true, false, false, false, false, true, false, false, true, false, false, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, false, true, false, true, false, true, true, true, false, false, false, false, false, true, false, true, true, false, true, true, false, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, false, true, false, false, false, true, true, true, false, false, false, false, false, true, false, true, true, false, true, true, false, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, true, true, true, false, true, false, true, true, true, false, false, true, false, true, false, false, false, false, false, false, true, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, true, true, true, false, true, false, true, true, true, false, true, true, false, true, false, false, false, false, false, false, true, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, true, false, true, true, true, true, true, false, true, false, false, true, true, true, false, false, false, true, true, true, true, true, false, true, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, true, true, false, true, true, true, true, true, false, true, false, false, true, true, true, false, false, false, true, true, true, true, true, true, true, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, true, false, false, true, false, true, true, false, true, false, false, true, true, true, true, true, false, true, false, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, false, false, true, false, false, true, false, true, true, false, true, false, false, true, true, true, true, true, false, true, false, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, false, false, false, true, true, true, true, true, false, false, true, false, false, false, true, false, false, false, false, false, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, true, false, false, false, true, true, true, true, true, false, false, true, false, false, false, true, false, false, false, false, false, true, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, false, true, true, true, false, true, true, true, false, false, false, false, false, false, true, true, true, false, true, true, false, true, false, true, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, false, true, true, true, false, true, true, true, false, false, false, false, false, true, true, true, true, false, true, true, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, false, false, false, false, true, false, true, false, true, true, false, false, true, true, false, false, true, true, true, false, true, true, true, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, false, false, false, true, false, true, true, true, true, false, false, true, true, false, false, true, true, true, false, true, true, true, false, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, false, false, false, true, true, true, false, false, false, false, true, true, true, true, true, true, true, true, false, false, false, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, false, false, false, false, true, true, false, false, false, false, true, true, true, true, true, true, true, true, false, false, false, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, true, false, false, true, false, false, true, true, true, false, true, true, true, true, false, false, false, true, false, false, true, false, true, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, false, false, false, false, false, true, true, true, false, true, true, true, true, false, false, false, true, false, false, true, false, true, true, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, true, true, false, false, false, false, true, false, true, true, false, true, true, true, false, true, true, false, false, false, false, false, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, true, true, false, false, false, false, true, false, true, true, false, true, true, true, false, true, true, false, false, false, false, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, false, true, false, true, true, false, true, true, true, true, false, true, false, false, true, true, true, true, true, false, true, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, true, false, true, false, true, true, false, true, true, true, true, false, true, false, true, true, true, true, true, true, false, true, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, true, false, false, false, true, false, false, false, false, false, false, true, true, false, true, true, true, true, false, false, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, true, true, false, true, true, true, true, false, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, false, true, false, true, true, false, false, false, true, false, false, true, false, true, true, false, false, true, true, true, true, false, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, false, false, true, false, true, true, false, false, false, true, false, false, true, false, true, true, false, false, true, true, true, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, false, true, false, false, false, false, false, true, true, true, false, true, false, true, true, false, false, false, true, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, true, false, true, false, false, false, false, false, true, true, true, false, false, false, true, true, false, false, false, true, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, true, true, true, true, false, false, true, false, false, false, false, true, true, false, false, false, true, true, false, true, true, true, true, true, false, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, true, true, true, false, true, false, false, false, false, true, true, false, false, false, true, true, false, true, true, true, true, true, false, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, true, false, false, true, false, false, true, false, true, true, false, false, false, true, false, false, false, false, false, false, true, false, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, true, false, false, true, false, false, true, false, true, true, false, true, false, true, false, false, false, false, false, false, true, false, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, true, false, false, false, true, false, true, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, false, true, false, false, true, false, false, true, false, false, true, false, false, true, true, false, false, false, true, false, true, true, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, true, true, false, false, false, false, true, true, false, true, false, false, false, false, true, false, true, true, false, false, true, true, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, true, true, false, false, false, false, true, true, false, true, false, false, false, false, true, false, true, true, true, false, true, true, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, false, false, true, true, false, true, false, true, true, true, true, true, true, true, true, true, false, true, false, false, false, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, false, false, false, true, false, false, true, true, false, true, false, true, true, true, true, true, true, true, true, true, false, true, false, false, false, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, true, false, true, true, true, false, false, true, false, false, true, true, true, false, false, true, true, true, true, true, false, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, false, true, true, true, false, false, true, false, false, true, false, true, false, false, true, true, true, true, true, false, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, true, true, true, false, true, false, true, false, false, true, false, false, true, true, false, true, false, true, false, true, false, false, true, false, false, false, true, true, false]);
            let codeword = BinVector::from_bools(&[false, false, true, true, true, false, true, false, true, false, false, true, false, false, true, true, false, true, false, true, false, false, false, false, true, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, true, false, true, true, true, true, false, true, true, false, true, true, false, false, false, true, false, true, true, true, false, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, true, false, true, true, true, true, false, true, true, false, true, true, false, false, false, true, true, true, true, true, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, true, false, true, false, true, false, false, true, true, true, false, false, false, false, false, true, false, false, false, false, false, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, false, true, false, true, false, false, true, true, true, false, false, false, false, false, false, false, false, false, false, false, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, true, true, false, false, true, false, true, true, true, false, true, true, false, false, true, false, false, false, true, false, true, true, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, false, true, true, true, false, false, true, false, true, true, true, false, true, true, false, false, true, false, false, false, true, false, true, true, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, false, false, true, false, true, true, true, false, true, true, false, false, false, false, false, true, true, false, true, true, true, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, false, false, true, false, true, true, true, false, true, true, false, false, false, false, false, true, true, false, true, true, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, false, true, false, false, false, false, false, true, true, false, true, true, true, true, true, false, false, false, false, false, false, true, true, false, true, false, true, true, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, false, false, false, false, true, true, false, true, true, true, true, true, false, false, false, false, false, false, true, true, false, true, false, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, false, false, true, false, false, false, false, false, true, false, true, true, false, false, true, false, true, true, true, false, false, false, true, true, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, false, false, false, false, false, true, true, true, true, false, false, true, false, true, true, true, false, false, false, true, true, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, false, false, false, false, false, true, true, true, false, false, true, false, true, true, false, true, false, true, true, true, true, false, true, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, true, false, false, false, false, false, true, true, true, false, false, true, false, true, true, false, true, false, true, true, true, true, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, false, false, true, true, false, false, false, true, true, true, true, true, true, true, false, false, false, true, true, false, true, true, false, true, false, false, true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false, false, true, true, false, false, false, true, true, true, true, true, true, true, false, false, true, true, true, false, true, true, false, true, false, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, false, true, true, true, true, false, true, true, true, true, false, false, true, true, false, true, false, true, true, true, false, true, false, true, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, true, true, true, true, false, true, true, true, true, false, false, true, true, false, true, false, true, true, true, false, true, false, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, false, true, false, true, false, false, false, true, true, true, true, true, true, true, true, false, true, true, true, true, true, false, false, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, false, true, false, false, false, true, true, true, true, true, true, true, true, false, true, true, true, true, false, false, false, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, true, false, true, false, true, false, false, true, false, true, true, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, true, true, true]);
            let codeword = BinVector::from_bools(&[true, false, true, false, true, false, true, false, false, true, false, true, true, false, false, true, false, false, false, false, false, false, true, false, false, false, false, false, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, false, true, true, false, false, true, false, false, true, true, false, false, true, true, false, false, false, true, true, false, true, false, false, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[true, true, false, true, true, false, false, true, false, false, true, true, false, true, true, true, false, false, false, true, true, false, true, false, false, false, false, true, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, true, false, false, true, false, false, false, true, true, false, true, true, false, false, false, true, true, true, false, false, true, true, true, true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, true, false, false, true, false, false, false, true, true, false, true, true, false, false, false, true, true, true, false, false, true, true, true, true, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, false, true, false, true, true, false, false, false, true, true, true, true, false, false, true, true, true, false, true, false, true, true, true, false, false, true]);
            let codeword = BinVector::from_bools(&[false, true, true, true, false, false, true, false, true, true, false, false, false, true, true, true, true, false, false, true, true, true, false, true, false, true, true, true, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, false, false, true, true, true, true, true, false, false, false, true, true, false, true, false, false, false, false, true, false, true, true, true, false, false, true, false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false, false, true, true, true, true, true, false, false, false, true, true, false, true, false, false, false, false, true, false, true, true, true, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, true, false, false, false, false, true, true, false, false, true, false, false, false, true, true, true, false, true, true, false, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, true, true, false, false, false, false, true, true, false, false, true, false, false, false, true, true, true, false, true, true, false, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, true, false, true, true, true, true, false, true, true, true, false, false, false, false, false, true, true, true, true, true, false, false, true, false, false, false, false, false, false]);
            let codeword = BinVector::from_bools(&[false, true, true, false, true, true, true, true, false, true, true, false, false, false, false, false, false, true, true, true, true, true, false, false, true, false, false, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, true, false, true, false, true, true, false, false, false, true, false, true, false, false, true, true, false, false, true, false, false, false, true, false, true, false, false, false, true, true]);
            let codeword = BinVector::from_bools(&[false, true, false, true, false, true, true, false, false, false, true, false, true, false, false, false, true, false, false, true, false, false, false, true, false, true, false, false, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, false, true, true, false, true, true, true, false, true, false, true, true, false, false, false, false, true, true, true, true, false, false, true, false, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[false, false, false, true, true, false, true, true, true, false, true, false, true, true, false, false, false, false, true, true, true, true, true, false, true, false, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, true, false, false, false, false, true, true, false, false, false, true, false, true, false, true, false, true, true, true, false, true, true, true, false, true, false, false, true, false]);
            let codeword = BinVector::from_bools(&[true, false, true, false, false, false, false, true, true, false, false, false, true, false, true, true, true, false, true, true, true, false, true, true, true, false, true, false, false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, false, false, false, false, false, true, true, false, false, true, true, true, true, true, false, false, true, true, false, false, false, true, true, false, false, true, false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, false, false, false, false, false, true, true, false, false, false, true, true, true, true, false, false, true, true, false, false, false, true, true, false, false, true, false, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, false, false, false, false, false, false, false, false, true, true, false, true, true, false, true, false, true, true, true, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, true, false, false, false, false, false, false, false, false, false, true, true, false, true, true, false, true, false, true, true, true, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, false, true, false, false, true, true, true, true, false, false, true, false, false, true, true, true, true, false, true, true, true, false, true, false, false, false]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, false, true, false, false, true, true, true, true, false, false, true, false, false, false, true, true, true, false, true, true, true, false, true, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, false, true, true, false, false, true, false, true, false, true, false, false, true, true, false, false, false, false, true, false, true, true, false, true, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, true, true, false, false, true, false, true, false, false, false, false, true, true, false, false, false, false, true, false, true, true, false, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, true, true, false, false, true, true, true, true, false, false, false, false, true, false, true, true, true, false, true, false, false, false, true, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, true, false, false, false, true, true, true, true, false, false, false, false, true, false, true, true, true, false, true, false, false, false, true, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, false, true, false, true, true, true, true, true, true, false, false, false, false, true, true, true, true, true, false, false, false, false, true, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[false, false, false, true, false, true, true, true, true, true, true, false, false, false, false, true, true, true, true, true, false, false, false, false, true, true, true, false, false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, true, false, false, false, true, true, true, false, false, false, true, false, false, true, true, true, true, true, true, true, false, false, false, true, false, true, true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true, false, false, false, false, true, true, false, false, false, true, false, false, true, true, true, true, true, true, true, false, false, false, true, false, true, true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, true, true, true, false, false, true, true, true, true, false, true, true, false, true, true, true, false, false, true, true, true, false, true, false, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, true, true, true, false, false, true, true, true, true, false, true, true, false, true, true, true, false, false, true, true, true, false, true, false, true, true, false, true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[false, false, true, false, false, false, false, false, true, true, true, false, false, true, true, true, false, true, false, false, true, true, true, true, false, false, true, false, true, false, true]);
            let codeword = BinVector::from_bools(&[false, false, true, false, false, false, false, false, true, true, true, false, false, true, true, true, false, true, false, false, true, true, true, true, false, false, true, false, true, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = HammingCode31_26;
            let randvec = BinVector::from_bools(&[true, false, false, false, false, true, false, true, false, true, true, false, true, true, true, true, true, true, true, true, false, true, true, false, false, true, true, false, false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, false, false, false, true, false, true, false, true, true, true, true, true, true, true, true, true, true, true, false, true, true, false, false, true, true, false, false, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

    #[test]
    fn test_generator_representation() {
        init();
        let generator_matrix = unsafe { GENERATOR_MATRIX.as_ref().unwrap() };
        let first_row = generator_matrix.get_window(0, 0, 1, generator_matrix.ncols());
        let vector = BinVector::from_bools(&[ true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true ]);
        assert_eq!(vector, first_row.as_vector());
    }

}
