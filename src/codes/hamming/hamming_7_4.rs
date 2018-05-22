use codes::BinaryCode;
use std::default::Default;
use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;
use std::sync::{Once,ONCE_INIT};
use std::boxed::Box;

use fnv::FnvHashMap;


pub struct HammingCode7_4;

static INIT: Once = ONCE_INIT;
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut SYNDROME_MAP: *const FnvHashMap<usize, [bool; 7]> = 0 as *const FnvHashMap<usize, [bool; 7]>;

fn init() {
    INIT.call_once(|| {
        unsafe {
            let matrix = Box::new(BinMatrix::new(vec![
                BinVector::from_bools(&[true, false, false, false, false, true, true]),
                BinVector::from_bools(&[false, true, false, false, true, false, true]),
                BinVector::from_bools(&[false, false, true, false, true, true, false]),
                BinVector::from_bools(&[false, false, false, true, true, true, true]),
                
            ]));
            GENERATOR_MATRIX = Box::into_raw(matrix);

            let matrix = Box::new(BinMatrix::new(vec![
                BinVector::from_bools(&[true, false, true, false, true, false, true]),
                BinVector::from_bools(&[false, true, true, false, false, true, true]),
                BinVector::from_bools(&[false, false, false, true, true, true, true]),
                
            ]));
            PARITY_MATRIX = Box::into_raw(matrix);

            let mut map = Box::new(FnvHashMap::with_capacity_and_hasher(8, Default::default()));
            map.insert(0, [false, false, false, false, false, false, false]); // 0 => (0, 0, 0, 0, 0, 0, 0)
            map.insert(1, [true, false, false, false, false, false, false]); // 1 => (1, 0, 0, 0, 0, 0, 0)
            map.insert(2, [false, true, false, false, false, false, false]); // 2 => (0, 1, 0, 0, 0, 0, 0)
            map.insert(3, [false, false, true, false, false, false, false]); // 3 => (0, 0, 1, 0, 0, 0, 0)
            map.insert(4, [false, false, false, true, false, false, false]); // 4 => (0, 0, 0, 1, 0, 0, 0)
            map.insert(5, [false, false, false, false, true, false, false]); // 5 => (0, 0, 0, 0, 1, 0, 0)
            map.insert(6, [false, false, false, false, false, true, false]); // 6 => (0, 0, 0, 0, 0, 1, 0)
            map.insert(7, [false, false, false, false, false, false, true]); // 7 => (0, 0, 0, 0, 0, 0, 1)
            
            SYNDROME_MAP = Box::into_raw(map);
        }
    });
}


impl BinaryCode<'static> for HammingCode7_4 {
    fn length(&self) -> usize {
        7
    }

    fn dimension(&self) -> usize {
        4
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
        codeword.truncate(4);
        codeword
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use m4ri_rust::friendly::BinVector;

    #[test]
    fn size() {
        let code = HammingCode7_4.generator_matrix();
        assert_eq!(code.ncols(), 7);
        assert_eq!(code.nrows(), 4);
    }

    #[test]
    fn decode() {
        let code = HammingCode7_4;

        let codeword = code.decode_to_message(&BinVector::from_elem(7, true));
        assert_eq!(codeword, BinVector::from_elem(4, true));

        let mut vec = BinVector::from_elem(7, true);
        vec.set(0, false);
        let codeword = code.decode_to_message(&vec);
        assert_eq!(codeword, BinVector::from_elem(4, true));

        let vec = code.decode_to_code(&BinVector::from_elem(7, false));
        assert_eq!(vec, BinVector::from_elem(7, false));
    }

    #[test]
    fn encode_decode_tests() {
        let code = HammingCode7_4;

        
        {
            let m = BinVector::from_bools(&[true, true, false, true]);
            let encoded = BinVector::from_bools(&[true, true, false, true, false, false, true]);
            assert_eq!(code.encode(&m), encoded);
            
            {
                let errored = BinVector::from_bools(&[true, true, true, true, false, false, true]);
                assert_eq!(&m, &code.decode_to_message(&errored), "decode to msg failed");
                assert_eq!(encoded, code.decode_to_code(&errored), "decode to code failed");
            }
            
        }
        
        {
            let m = BinVector::from_bools(&[true, false, false, true]);
            let encoded = BinVector::from_bools(&[true, false, false, true, true, false, false]);
            assert_eq!(code.encode(&m), encoded);
            
            {
                let errored = BinVector::from_bools(&[true, false, false, true, true, false, true]);
                assert_eq!(&m, &code.decode_to_message(&errored), "decode to msg failed");
                assert_eq!(encoded, code.decode_to_code(&errored), "decode to code failed");
            }
            
        }
        
        {
            let m = BinVector::from_bools(&[true, true, false, false]);
            let encoded = BinVector::from_bools(&[true, true, false, false, true, true, false]);
            assert_eq!(code.encode(&m), encoded);
            
            {
                let errored = BinVector::from_bools(&[true, true, false, false, false, true, false]);
                assert_eq!(&m, &code.decode_to_message(&errored), "decode to msg failed");
                assert_eq!(encoded, code.decode_to_code(&errored), "decode to code failed");
            }
            
        }
        
        {
            let m = BinVector::from_bools(&[false, true, true, true]);
            let encoded = BinVector::from_bools(&[false, true, true, true, true, false, false]);
            assert_eq!(code.encode(&m), encoded);
            
            {
                let errored = BinVector::from_bools(&[false, true, true, true, false, false, false]);
                assert_eq!(&m, &code.decode_to_message(&errored), "decode to msg failed");
                assert_eq!(encoded, code.decode_to_code(&errored), "decode to code failed");
            }
            
        }
        
        {
            let m = BinVector::from_bools(&[false, true, true, false]);
            let encoded = BinVector::from_bools(&[false, true, true, false, false, true, true]);
            assert_eq!(code.encode(&m), encoded);
            
            {
                let errored = BinVector::from_bools(&[false, true, true, false, false, false, true]);
                assert_eq!(&m, &code.decode_to_message(&errored), "decode to msg failed");
                assert_eq!(encoded, code.decode_to_code(&errored), "decode to code failed");
            }
            
        }
        
        {
            let m = BinVector::from_bools(&[true, true, true, false]);
            let encoded = BinVector::from_bools(&[true, true, true, false, false, false, false]);
            assert_eq!(code.encode(&m), encoded);
            
            {
                let errored = BinVector::from_bools(&[true, true, true, false, true, false, false]);
                assert_eq!(&m, &code.decode_to_message(&errored), "decode to msg failed");
                assert_eq!(encoded, code.decode_to_code(&errored), "decode to code failed");
            }
            
        }
        
        {
            let m = BinVector::from_bools(&[false, false, true, true]);
            let encoded = BinVector::from_bools(&[false, false, true, true, false, false, true]);
            assert_eq!(code.encode(&m), encoded);
            
            {
                let errored = BinVector::from_bools(&[false, false, true, false, false, false, true]);
                assert_eq!(&m, &code.decode_to_message(&errored), "decode to msg failed");
                assert_eq!(encoded, code.decode_to_code(&errored), "decode to code failed");
            }
            
        }
        
        {
            let m = BinVector::from_bools(&[false, false, true, false]);
            let encoded = BinVector::from_bools(&[false, false, true, false, true, true, false]);
            assert_eq!(code.encode(&m), encoded);
            
            {
                let errored = BinVector::from_bools(&[false, false, true, true, true, true, false]);
                assert_eq!(&m, &code.decode_to_message(&errored), "decode to msg failed");
                assert_eq!(encoded, code.decode_to_code(&errored), "decode to code failed");
            }
            
        }
        
        {
            let m = BinVector::from_bools(&[true, false, true, false]);
            let encoded = BinVector::from_bools(&[true, false, true, false, true, false, true]);
            assert_eq!(code.encode(&m), encoded);
            
            {
                let errored = BinVector::from_bools(&[true, false, true, false, true, true, true]);
                assert_eq!(&m, &code.decode_to_message(&errored), "decode to msg failed");
                assert_eq!(encoded, code.decode_to_code(&errored), "decode to code failed");
            }
            
        }
        
        {
            let m = BinVector::from_bools(&[true, false, true, true]);
            let encoded = BinVector::from_bools(&[true, false, true, true, false, true, false]);
            assert_eq!(code.encode(&m), encoded);
            
            {
                let errored = BinVector::from_bools(&[true, true, true, true, false, true, false]);
                assert_eq!(&m, &code.decode_to_message(&errored), "decode to msg failed");
                assert_eq!(encoded, code.decode_to_code(&errored), "decode to code failed");
            }
            
        }
        
        {
            let m = BinVector::from_bools(&[false, true, false, true]);
            let encoded = BinVector::from_bools(&[false, true, false, true, false, true, false]);
            assert_eq!(code.encode(&m), encoded);
            
            {
                let errored = BinVector::from_bools(&[false, true, true, true, false, true, false]);
                assert_eq!(&m, &code.decode_to_message(&errored), "decode to msg failed");
                assert_eq!(encoded, code.decode_to_code(&errored), "decode to code failed");
            }
            
        }
        
        {
            let m = BinVector::from_bools(&[true, false, false, false]);
            let encoded = BinVector::from_bools(&[true, false, false, false, false, true, true]);
            assert_eq!(code.encode(&m), encoded);
            
            {
                let errored = BinVector::from_bools(&[true, false, false, true, false, true, true]);
                assert_eq!(&m, &code.decode_to_message(&errored), "decode to msg failed");
                assert_eq!(encoded, code.decode_to_code(&errored), "decode to code failed");
            }
            
        }
        
        {
            let m = BinVector::from_bools(&[false, true, false, false]);
            let encoded = BinVector::from_bools(&[false, true, false, false, true, false, true]);
            assert_eq!(code.encode(&m), encoded);
            
            {
                let errored = BinVector::from_bools(&[false, true, false, true, true, false, true]);
                assert_eq!(&m, &code.decode_to_message(&errored), "decode to msg failed");
                assert_eq!(encoded, code.decode_to_code(&errored), "decode to code failed");
            }
            
        }
        
        {
            let m = BinVector::from_bools(&[true, true, true, true]);
            let encoded = BinVector::from_bools(&[true, true, true, true, true, true, true]);
            assert_eq!(code.encode(&m), encoded);
            
            {
                let errored = BinVector::from_bools(&[true, true, true, true, true, false, true]);
                assert_eq!(&m, &code.decode_to_message(&errored), "decode to msg failed");
                assert_eq!(encoded, code.decode_to_code(&errored), "decode to code failed");
            }
            
        }
        
        {
            let m = BinVector::from_bools(&[false, false, false, false]);
            let encoded = BinVector::from_bools(&[false, false, false, false, false, false, false]);
            assert_eq!(code.encode(&m), encoded);
            
            {
                let errored = BinVector::from_bools(&[false, false, false, false, true, false, false]);
                assert_eq!(&m, &code.decode_to_message(&errored), "decode to msg failed");
                assert_eq!(encoded, code.decode_to_code(&errored), "decode to code failed");
            }
            
        }
        
        {
            let m = BinVector::from_bools(&[false, false, false, true]);
            let encoded = BinVector::from_bools(&[false, false, false, true, true, true, true]);
            assert_eq!(code.encode(&m), encoded);
            
            {
                let errored = BinVector::from_bools(&[false, false, false, true, false, true, true]);
                assert_eq!(&m, &code.decode_to_message(&errored), "decode to msg failed");
                assert_eq!(encoded, code.decode_to_code(&errored), "decode to code failed");
            }
            
        }
        
    }

}
