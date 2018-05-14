use codes::BinaryCode;
use std::default::Default;
use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use fnv::FnvHashMap;


pub struct HammingCode3_1;



lazy_static! {
    static ref GENERATOR_MATRIX: BinMatrix = BinMatrix::new(vec![
      BinVector::from_bools(&[true, true, true]),

    ]);
    static ref PARITY_MATRIX: BinMatrix = BinMatrix::new(vec![
      BinVector::from_bools(&[true, false, true]),
      BinVector::from_bools(&[false, true, true]),

    ]);

    /// Map from He to e
    static ref SYNDROME_MAP: FnvHashMap<usize, [bool; 3]> = {
        let mut map = FnvHashMap::with_capacity_and_hasher(4, Default::default());
        map.insert(0, [false, false, false]); // 0 => (0, 0, 0)
        map.insert(1, [true, false, false]); // 1 => (1, 0, 0)
        map.insert(2, [false, true, false]); // 2 => (0, 1, 0)
        map.insert(3, [false, false, true]); // 3 => (0, 0, 1)
        
        map
    };
}



impl BinaryCode for HammingCode3_1 {
    fn length() -> usize {
        3
    }

    fn dimension() -> usize {
        1
    }

    fn generator_matrix(&self) -> &'static BinMatrix {
        &GENERATOR_MATRIX
    }

    fn parity_check_matrix(&self) -> &'static BinMatrix {
        &PARITY_MATRIX
    }

    fn decode_to_code(&self, c: BinVector) -> BinVector {
        debug_assert_eq!(c.len(), Self::length());
        let he = self.parity_check_matrix() * &c;
        let error = BinVector::from_bools(&SYNDROME_MAP[&(he.as_u32() as usize)]);
        c + error
    }

    fn decode_to_message(&self, c: BinVector) -> BinVector {
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

        let codeword = code.decode_to_message(BinVector::from_elem(3, true));
        assert_eq!(codeword, BinVector::from_elem(1, true));

        let mut vec = BinVector::from_elem(3, true);
        vec.set(0, false);
        let codeword = code.decode_to_message(vec);
        assert_eq!(codeword, BinVector::from_elem(1, true));

        let vec = code.decode_to_code(BinVector::from_elem(3, false));
        assert_eq!(vec, BinVector::from_elem(3, false));
    }

    #[test]
    fn encode_decode_tests() {
        let code = HammingCode3_1;

        
        {
            let m = BinVector::from_bools(&[true]);
            let encoded = BinVector::from_bools(&[true, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);
            
            {
                let errored = BinVector::from_bools(&[true, false, true]);
                assert_eq!(m, code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded, code.decode_to_code(errored), "decode to code failed");
            }
            
        }
        
                
    }

}
