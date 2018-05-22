use codes::BinaryCode;
use std::default::Default;
use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;
use std::sync::{Once,ONCE_INIT};
use std::boxed::Box;

use fnv::FnvHashMap;


pub struct HammingCode3_1;

static INIT: Once = ONCE_INIT;
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<usize, [bool; 3]> = 0 as *const FnvHashMap<usize, [bool; 3]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::new(vec![
                BinVector::from_bools(&[true, true, true]),
                
            ]));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::new(vec![
                BinVector::from_bools(&[true, false, true]),
                BinVector::from_bools(&[false, true, true]),
                
            ]));
            PARITY_MATRIX = Box::into_raw(matrix);

            let mut map = Box::new(FnvHashMap::with_capacity_and_hasher(4, Default::default()));
            map.insert(0, [false, false, false]); // 0 => (0, 0, 0)
            map.insert(1, [true, false, false]); // 1 => (1, 0, 0)
            map.insert(2, [false, true, false]); // 2 => (0, 1, 0)
            map.insert(3, [false, false, true]); // 3 => (0, 0, 1)
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}


impl BinaryCode<'static> for HammingCode3_1 {
    fn length(&self) -> usize {
        3
    }

    fn dimension(&self) -> usize {
        1
    }

    fn generator_matrix(&self) -> &'static BinMatrix {
        init();
        unsafe {
            GENERATOR_MATRIX.as_ref().unwrap()
        }
    }

    fn parity_check_matrix(&self) -> &'static BinMatrix {
        init();
        unsafe {
            PARITY_MATRIX.as_ref().unwrap()
        }
    }

    fn decode_to_code(&self, c: &BinVector) -> BinVector {
        init();
        let map = unsafe {
            SYNDROME_MAP.as_ref().unwrap()
        };
        debug_assert_eq!(c.len(), self.length());
        let he = self.parity_check_matrix() * c;
        let error = BinVector::from_bools(&map[&(he.as_u64() as usize)]);
        debug_assert_eq!(error.len(), self.length());
        let result = c + &error;
        debug_assert_eq!(result.len(), self.length());
        result
    }

    fn decode_to_message(&self, c: &BinVector) -> BinVector {
        
        let mut codeword = self.decode_to_code(c);
        codeword.truncate(1);
        codeword
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use m4ri_rust::friendly::BinVector;

    #[test]
    fn size() {
        let code = HammingCode3_1.generator_matrix();
        assert_eq!(code.ncols(), 3);
        assert_eq!(code.nrows(), 1);
    }

    #[test]
    fn decode() {
        let code = HammingCode3_1;

        let codeword = code.decode_to_message(&BinVector::from_elem(3, true));
        assert_eq!(codeword, BinVector::from_elem(1, true));

        let mut vec = BinVector::from_elem(3, true);
        vec.set(0, false);
        let codeword = code.decode_to_message(&vec);
        assert_eq!(codeword, BinVector::from_elem(1, true));

        let vec = code.decode_to_code(&BinVector::from_elem(3, false));
        assert_eq!(vec, BinVector::from_elem(3, false));
    }

    #[test]
    fn encode_decode_tests() {
        let code = HammingCode3_1;

        
        {
            let m = BinVector::from_bools(&[true]);
            let encoded = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.encode(&m), encoded);
            
            {
                let errored = BinVector::from_bools(&[true, false, true]);
                assert_eq!(&m, &code.decode_to_message(&errored), "decode to msg failed");
                assert_eq!(encoded, code.decode_to_code(&errored), "decode to code failed");
            }
            
        }
        
        {
            let m = BinVector::from_bools(&[false]);
            let encoded = BinVector::from_bools(&[false, false, false]);
            assert_eq!(code.encode(&m), encoded);
            
            {
                let errored = BinVector::from_bools(&[false, false, true]);
                assert_eq!(&m, &code.decode_to_message(&errored), "decode to msg failed");
                assert_eq!(encoded, code.decode_to_code(&errored), "decode to code failed");
            }
            
        }
        
    }

}
