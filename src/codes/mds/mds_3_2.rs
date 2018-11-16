use crate::codes::BinaryCode;
use std::default::Default;
use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;
use std::sync::{Once,ONCE_INIT};
use std::boxed::Box;

use fnv::FnvHashMap;

/// [3, 2] Mds code
///
/// Decodes using Syndrome decoding
#[derive(Clone)]
pub struct MdsCode3_2;

static INIT: Once = ONCE_INIT;
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<usize, [bool; 3]> = 0 as *const FnvHashMap<usize, [bool; 3]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::new(vec![
                BinVector::from_bools(&[true, false, true]),
                BinVector::from_bools(&[false, true, false]),
                
            ]));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::new(vec![
                BinVector::from_bools(&[true, false, true]),
                
            ]));
            PARITY_MATRIX = Box::into_raw(matrix);

            let mut map = Box::new(FnvHashMap::with_capacity_and_hasher(2, Default::default()));
            map.insert(0, [false, false, false]); // 0 => (0, 0, 0)
            map.insert(1, [true, false, false]); // 1 => (1, 0, 0)
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}


impl BinaryCode for MdsCode3_2 {
    fn name(&self) -> String {
        "[3, 2] Mds code".to_owned()
    }

    fn length(&self) -> usize {
        3
    }

    fn dimension(&self) -> usize {
        2
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
        let he = self.parity_check_matrix() * c;
        let error = BinVector::from_bools(&map[&(he.as_u64() as usize)]);
        debug_assert_eq!(error.len(), self.length(), "internal: the error vector is of the wrong length");
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length(), "internal: the result vector is of the wrong length");
        debug_assert_eq!((self.parity_check_matrix() * &result).count_ones(), 0);
        Ok(result)
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        
        let mut codeword = self.decode_to_code(c)?;
        codeword.truncate(2);
        Ok(codeword)
        
    }

    
}

#[cfg(test)]
mod tests {
    use super::*;
    use m4ri_rust::friendly::BinVector;

    #[test]
    fn size() {
        let code = MdsCode3_2.generator_matrix();
        assert_eq!(code.ncols(), 3);
        assert_eq!(code.nrows(), 2);
    }

    #[test]
    fn random_decode_tests() {

        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, true]);
            let codeword = BinVector::from_bools(&[true, false, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[false, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, true]);
            let codeword = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, true, false]);
            let codeword = BinVector::from_bools(&[false, true, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
        {
            let code = MdsCode3_2;
            let randvec = BinVector::from_bools(&[true, false, false]);
            let codeword = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.decode_to_code(&randvec), Ok(codeword));
        }
        
    }

}
