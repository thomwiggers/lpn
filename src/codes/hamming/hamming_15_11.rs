use codes::BinaryCode;
use std::default::Default;
use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use fnv::FnvHashMap;


pub struct HammingCode15_11;



lazy_static! {
    static ref GENERATOR_MATRIX: BinMatrix = BinMatrix::new(vec![
      BinVector::from_bools(&[true, false, false, false, false, false, false, false, false, false, false, false, false, true, true]),
      BinVector::from_bools(&[false, true, false, false, false, false, false, false, false, false, false, false, true, false, true]),
      BinVector::from_bools(&[false, false, true, false, false, false, false, false, false, false, false, false, true, true, false]),
      BinVector::from_bools(&[false, false, false, true, false, false, false, false, false, false, true, false, false, false, true]),
      BinVector::from_bools(&[false, false, false, false, true, false, false, false, false, false, true, false, false, true, false]),
      BinVector::from_bools(&[false, false, false, false, false, true, false, false, false, false, true, false, true, false, false]),
      BinVector::from_bools(&[false, false, false, false, false, false, true, false, false, false, true, false, true, true, true]),
      BinVector::from_bools(&[false, false, false, false, false, false, false, true, false, false, true, false, true, true, false]),
      BinVector::from_bools(&[false, false, false, false, false, false, false, false, true, false, true, false, true, false, true]),
      BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, true, true, false, false, true, true]),
      BinVector::from_bools(&[false, false, false, false, false, false, false, false, false, false, false, true, true, true, true]),

    ]);
    static ref PARITY_MATRIX: BinMatrix = BinMatrix::new(vec![
      BinVector::from_bools(&[true, false, true, false, true, false, true, false, true, false, true, false, true, false, true]),
      BinVector::from_bools(&[false, true, true, false, false, true, true, false, false, true, true, false, false, true, true]),
      BinVector::from_bools(&[false, false, false, true, true, true, true, false, false, false, false, true, true, true, true]),
      BinVector::from_bools(&[false, false, false, false, false, false, false, true, true, true, true, true, true, true, true]),

    ]);

    /// Map from He to e
    static ref SYNDROME_MAP: FnvHashMap<usize, [bool; 15]> = {
        let mut map = FnvHashMap::with_capacity_and_hasher(16, Default::default());
        map.insert(0, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 0 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
        map.insert(1, [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 1 => (1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
        map.insert(2, [false, true, false, false, false, false, false, false, false, false, false, false, false, false, false]); // 2 => (0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
        map.insert(3, [false, false, true, false, false, false, false, false, false, false, false, false, false, false, false]); // 3 => (0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
        map.insert(4, [false, false, false, true, false, false, false, false, false, false, false, false, false, false, false]); // 4 => (0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
        map.insert(5, [false, false, false, false, true, false, false, false, false, false, false, false, false, false, false]); // 5 => (0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
        map.insert(6, [false, false, false, false, false, true, false, false, false, false, false, false, false, false, false]); // 6 => (0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0)
        map.insert(7, [false, false, false, false, false, false, true, false, false, false, false, false, false, false, false]); // 7 => (0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0)
        map.insert(8, [false, false, false, false, false, false, false, true, false, false, false, false, false, false, false]); // 8 => (0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0)
        map.insert(9, [false, false, false, false, false, false, false, false, true, false, false, false, false, false, false]); // 9 => (0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
        map.insert(10, [false, false, false, false, false, false, false, false, false, true, false, false, false, false, false]); // 10 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0)
        map.insert(11, [false, false, false, false, false, false, false, false, false, false, true, false, false, false, false]); // 11 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0)
        map.insert(12, [false, false, false, false, false, false, false, false, false, false, false, true, false, false, false]); // 12 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
        map.insert(13, [false, false, false, false, false, false, false, false, false, false, false, false, true, false, false]); // 13 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
        map.insert(14, [false, false, false, false, false, false, false, false, false, false, false, false, false, true, false]); // 14 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
        map.insert(15, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true]); // 15 => (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)

        map
    };
}



impl BinaryCode for HammingCode15_11 {
    fn length() -> usize {
        15
    }

    fn dimension() -> usize {
        11
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
        debug_assert_eq!(error.len(), Self::length());
        let result = c + error;
        debug_assert_eq!(result.len(), Self::length());
        result
    }

    fn decode_to_message(&self, c: BinVector) -> BinVector {

        let codeword = self.decode_to_code(c);
        let mut new_codeword = BinVector::with_capacity(11);
        new_codeword.push(codeword[0]);
        new_codeword.push(codeword[1]);
        new_codeword.push(codeword[2]);
        new_codeword.push(codeword[3]);
        new_codeword.push(codeword[4]);
        new_codeword.push(codeword[5]);
        new_codeword.push(codeword[6]);
        new_codeword.push(codeword[7]);
        new_codeword.push(codeword[8]);
        new_codeword.push(codeword[9]);
        new_codeword.push(codeword[11]);

        new_codeword

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use m4ri_rust::friendly::BinVector;

    #[test]
    fn size() {
        let code = HammingCode15_11.generator_matrix();
        assert_eq!(code.ncols(), 15);
        assert_eq!(code.nrows(), 11);
    }

    #[test]
    fn decode() {
        let code = HammingCode15_11;

        let codeword = code.decode_to_message(BinVector::from_elem(15, true));
        assert_eq!(codeword, BinVector::from_elem(11, true));

        let mut vec = BinVector::from_elem(15, true);
        vec.set(0, false);
        let codeword = code.decode_to_message(vec);
        assert_eq!(codeword, BinVector::from_elem(11, true));

        let vec = code.decode_to_code(BinVector::from_elem(15, false));
        assert_eq!(vec, BinVector::from_elem(15, false));
    }

    #[test]
    fn encode_decode_tests() {
        let code = HammingCode15_11;


        {
            let m = BinVector::from_bools(&[false, true, true, true, false, false, false, true, true, false, false]);
            let encoded = BinVector::from_bools(&[false, true, true, true, false, false, false, true, true, false, true, false, false, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, true, true, false, false, false, true, true, false, true, false, false, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, false, false, false, true, true, true, false, false, true]);
            let encoded = BinVector::from_bools(&[true, false, false, false, false, true, true, true, false, false, true, true, false, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, false, false, false, true, true, true, true, false, true, true, false, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, true, true, true, false, true, false, true, false, false]);
            let encoded = BinVector::from_bools(&[false, true, true, true, true, false, true, false, true, false, false, false, false, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, false, true, true, false, true, false, true, false, false, false, false, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, true, false, false, true, true, false, false, true, false]);
            let encoded = BinVector::from_bools(&[false, false, true, false, false, true, true, false, false, true, true, false, true, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, false, false, false, true, true, false, false, true, true, false, true, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, true, true, true, false, true, true, true, false, true]);
            let encoded = BinVector::from_bools(&[true, false, true, true, true, false, true, true, true, false, true, true, true, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, true, true, true, false, true, true, true, false, true, false, true, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, false, true, false, true, true, false, false, false, true]);
            let encoded = BinVector::from_bools(&[true, true, false, true, false, true, true, false, false, false, true, true, false, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, false, true, false, true, true, false, false, false, true, false, false, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, true, true, false, true, false, true, true, false, false]);
            let encoded = BinVector::from_bools(&[false, true, true, true, false, true, false, true, true, false, false, false, true, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, false, true, false, true, false, true, true, false, false, false, true, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, false, true, true, false, true, true, false, true, false]);
            let encoded = BinVector::from_bools(&[true, false, false, true, true, false, true, true, false, true, true, false, false, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, false, true, true, false, true, false, false, true, true, false, false, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, false, false, false, false, false, true, true, false, false]);
            let encoded = BinVector::from_bools(&[true, false, false, false, false, false, false, true, true, false, false, false, false, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, false, false, true, false, false, true, true, false, false, false, false, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, false, false, false, true, true, true, false, false, false]);
            let encoded = BinVector::from_bools(&[true, true, false, false, false, true, true, true, false, false, true, false, false, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, false, false, false, true, true, true, true, false, true, false, false, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, false, true, false, true, true, false, true, false, true]);
            let encoded = BinVector::from_bools(&[true, false, false, true, false, true, true, false, true, false, false, true, false, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, false, true, false, true, true, false, false, false, false, true, false, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, true, true, true, false, false, false, true, false, false]);
            let encoded = BinVector::from_bools(&[false, false, true, true, true, false, false, false, true, false, true, false, false, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, true, true, true, false, false, false, true, false, false, false, false, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, true, true, true, false, false, false, true, false, true]);
            let encoded = BinVector::from_bools(&[true, true, true, true, true, false, false, false, true, false, true, true, false, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, false, true, true, false, false, false, true, false, true, true, false, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, true, true, true, true, false, false, false, false, true]);
            let encoded = BinVector::from_bools(&[true, false, true, true, true, true, false, false, false, false, true, true, true, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, true, true, true, true, false, false, true, false, true, true, true, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, false, true, false, false, false, false, false, false, false]);
            let encoded = BinVector::from_bools(&[true, true, false, true, false, false, false, false, false, false, true, false, true, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, false, true, false, false, true, false, false, false, true, false, true, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, false, true, true, true, false, true, false, false, false]);
            let encoded = BinVector::from_bools(&[false, true, false, true, true, true, false, true, false, false, false, false, true, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, true, true, true, true, false, true, false, false, false, false, true, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, false, false, false, false, true, true, true, true, true]);
            let encoded = BinVector::from_bools(&[false, true, false, false, false, false, true, true, true, true, false, true, true, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, false, false, false, true, true, true, true, true, false, true, true, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, false, true, false, false, false, false, true, true, false]);
            let encoded = BinVector::from_bools(&[true, true, false, true, false, false, false, false, true, true, true, false, false, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, true, true, false, false, false, false, true, true, true, false, false, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, true, false, true, true, true, true, false, false, false]);
            let encoded = BinVector::from_bools(&[false, true, true, false, true, true, true, true, false, false, false, false, true, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, true, false, true, true, true, true, false, false, true, false, true, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, true, true, true, false, false, false, true, true, true]);
            let encoded = BinVector::from_bools(&[false, true, true, true, true, false, false, false, true, true, false, true, false, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, true, true, true, false, false, false, true, true, false, true, false, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, true, true, true, false, true, false, true, false, false]);
            let encoded = BinVector::from_bools(&[true, true, true, true, true, false, true, false, true, false, false, false, false, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, true, true, true, false, true, false, true, false, false, false, false, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, false, false, true, true, false, true, true, false, true]);
            let encoded = BinVector::from_bools(&[true, true, false, false, true, true, false, true, true, false, false, true, true, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, false, false, true, true, false, true, true, false, false, true, true, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, true, false, true, false, true, false, false, true, false]);
            let encoded = BinVector::from_bools(&[true, false, true, false, true, false, true, false, false, true, true, false, false, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, true, false, true, false, true, false, false, true, true, false, true, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, true, true, false, false, false, false, false, false, false]);
            let encoded = BinVector::from_bools(&[true, false, true, true, false, false, false, false, false, false, true, false, true, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, true, true, false, true, false, false, false, false, true, false, true, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, true, false, false, false, false, false, false, false, false]);
            let encoded = BinVector::from_bools(&[false, true, true, false, false, false, false, false, false, false, false, false, false, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, false, false, false, false, false, false, false, false, false, false, false, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, false, false, false, true, true, true, true, true, false]);
            let encoded = BinVector::from_bools(&[false, false, false, false, false, true, true, true, true, true, true, false, false, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, false, false, false, true, true, true, true, true, true, true, false, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, true, false, true, true, false, true, true, true, true]);
            let encoded = BinVector::from_bools(&[false, false, true, false, true, true, false, true, true, true, true, true, true, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, true, false, true, true, false, true, true, true, true, true, true, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, false, false, false, true, false, false, false, true, true]);
            let encoded = BinVector::from_bools(&[true, false, false, false, false, true, false, false, false, true, false, true, false, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, false, false, false, true, false, false, true, true, false, true, false, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, false, true, false, false, true, false, true, false, true]);
            let encoded = BinVector::from_bools(&[true, false, false, true, false, false, true, false, true, false, true, true, true, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, false, true, false, false, true, false, true, false, true, true, true, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, false, false, true, true, false, false, true, true, true]);
            let encoded = BinVector::from_bools(&[false, true, false, false, true, true, false, false, true, true, false, true, false, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, false, false, true, true, false, false, false, true, false, true, false, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, false, true, false, false, false, false, true, true, true]);
            let encoded = BinVector::from_bools(&[false, true, false, true, false, false, false, false, true, true, true, true, true, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, false, true, false, false, false, false, true, true, false, true, true, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, true, true, true, false, true, false, false, true, true]);
            let encoded = BinVector::from_bools(&[true, true, true, true, true, false, true, false, false, true, false, true, false, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, true, true, true, false, true, false, true, true, false, true, false, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, true, true, true, false, true, true, false, true, true]);
            let encoded = BinVector::from_bools(&[true, true, true, true, true, false, true, true, false, true, true, true, true, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, true, true, true, false, true, true, false, false, true, true, true, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, true, false, false, false, false, false, true, false, true]);
            let encoded = BinVector::from_bools(&[false, true, true, false, false, false, false, false, true, false, true, true, false, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, false, false, false, false, false, false, true, false, true, true, false, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, false, false, false, true, true, false, false, true, true]);
            let encoded = BinVector::from_bools(&[true, true, false, false, false, true, true, false, false, true, true, true, false, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, false, false, true, true, true, false, false, true, true, true, false, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, false, false, true, true, false, false, true, false, false]);
            let encoded = BinVector::from_bools(&[true, false, false, false, true, true, false, false, true, false, true, false, false, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, false, false, true, true, false, false, true, false, true, false, false, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, true, true, true, true, true, false, false, false, true]);
            let encoded = BinVector::from_bools(&[true, false, true, true, true, true, true, false, false, false, false, true, false, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, true, true, true, true, true, false, false, false, false, true, false, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, false, false, true, false, false, false, true, true, false]);
            let encoded = BinVector::from_bools(&[true, false, false, false, true, false, false, false, true, true, true, false, true, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, true, false, true, false, false, false, true, true, true, false, true, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, true, false, false, true, false, true, false, false, false]);
            let encoded = BinVector::from_bools(&[false, true, true, false, false, true, false, true, false, false, false, false, false, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, true, false, false, true, false, true, false, false, false, false, false, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, true, false, false, true, true, false, true, true, true]);
            let encoded = BinVector::from_bools(&[false, true, true, false, false, true, true, false, true, true, false, true, false, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, true, false, false, true, true, false, true, true, false, true, false, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, false, false, true, true, false, true, true, true, true]);
            let encoded = BinVector::from_bools(&[true, false, false, false, true, true, false, true, true, true, true, true, false, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, false, false, true, true, false, true, true, false, true, true, false, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, true, true, true, true, false, true, false, false, true]);
            let encoded = BinVector::from_bools(&[true, false, true, true, true, true, false, true, false, false, false, true, false, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, true, true, true, true, false, true, false, false, false, true, true, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, true, true, true, true, true, false, false, false, false]);
            let encoded = BinVector::from_bools(&[false, true, true, true, true, true, true, false, false, false, false, false, false, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, true, true, true, true, true, false, false, false, false, true, false, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, false, false, false, true, true, false, true, true, false]);
            let encoded = BinVector::from_bools(&[true, false, false, false, false, true, true, false, true, true, false, false, true, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, false, false, false, true, false, false, true, true, false, false, true, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, false, false, true, true, false, false, false, true, false]);
            let encoded = BinVector::from_bools(&[false, false, false, false, true, true, false, false, false, true, true, false, true, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, false, true, true, true, false, false, false, true, true, false, true, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, true, false, true, true, true, true, true, false, true]);
            let encoded = BinVector::from_bools(&[false, true, true, false, true, true, true, true, true, false, true, true, true, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, true, false, true, true, true, true, true, false, true, true, true, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, false, true, true, true, false, false, false, false, false]);
            let encoded = BinVector::from_bools(&[true, false, false, true, true, true, false, false, false, false, true, false, true, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, false, true, true, true, false, false, true, false, true, false, true, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, false, true, true, true, true, false, true, false, true]);
            let encoded = BinVector::from_bools(&[true, false, false, true, true, true, true, false, true, false, true, true, false, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, false, true, true, true, true, false, false, false, true, true, false, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, false, false, true, false, false, true, false, false, true]);
            let encoded = BinVector::from_bools(&[true, true, false, false, true, false, false, true, false, false, false, true, true, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, false, false, true, false, false, true, false, false, true, true, true, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, true, false, true, false, false, true, true, false, true]);
            let encoded = BinVector::from_bools(&[false, false, true, false, true, false, false, true, true, false, true, true, false, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, true, false, true, false, false, true, true, false, true, false, false, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, false, true, true, true, true, true, false, false, false]);
            let encoded = BinVector::from_bools(&[false, false, false, true, true, true, true, true, false, false, true, false, true, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, false, true, true, true, true, true, false, false, true, false, false, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, false, true, false, true, false, true, false, true, true]);
            let encoded = BinVector::from_bools(&[true, true, false, true, false, true, false, true, false, true, false, true, false, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, false, true, false, true, false, true, false, true, false, true, true, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, true, false, false, false, false, false, false, true, false]);
            let encoded = BinVector::from_bools(&[false, false, true, false, false, false, false, false, false, true, true, false, true, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, true, false, false, false, false, false, true, true, true, false, true, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, true, false, false, true, false, true, true, true, false]);
            let encoded = BinVector::from_bools(&[true, false, true, false, false, true, false, true, true, true, false, false, false, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, true, false, false, true, false, true, true, false, false, false, false, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, true, false, false, true, true, true, true, false, true]);
            let encoded = BinVector::from_bools(&[true, false, true, false, false, true, true, true, true, false, false, true, false, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, true, false, false, true, true, true, true, false, false, false, false, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, false, true, true, false, true, true, true, false, false]);
            let encoded = BinVector::from_bools(&[false, false, false, true, true, false, true, true, true, false, true, false, true, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, false, true, true, false, false, true, true, false, true, false, true, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, true, true, false, false, false, true, false, false, false]);
            let encoded = BinVector::from_bools(&[true, false, true, true, false, false, false, true, false, false, false, false, false, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, true, true, false, false, false, true, false, true, false, false, false, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, false, false, false, false, true, true, false, false, false]);
            let encoded = BinVector::from_bools(&[true, false, false, false, false, false, true, true, false, false, false, false, false, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, false, false, false, false, false, true, false, false, false, false, false, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, false, false, false, true, false, true, true, true, false]);
            let encoded = BinVector::from_bools(&[true, true, false, false, false, true, false, true, true, true, false, false, false, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, false, false, false, false, false, true, true, true, false, false, false, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, false, false, true, false, false, false, true, true, false]);
            let encoded = BinVector::from_bools(&[false, false, false, false, true, false, false, false, true, true, true, false, true, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, false, false, false, false, false, false, true, true, true, false, true, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, false, true, true, false, true, true, true, true, false]);
            let encoded = BinVector::from_bools(&[false, true, false, true, true, false, true, true, true, true, false, false, false, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, false, true, true, true, true, true, true, true, false, false, false, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, true, false, false, true, true, true, true, false, false]);
            let encoded = BinVector::from_bools(&[false, false, true, false, false, true, true, true, true, false, false, false, true, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, true, false, false, true, true, true, true, false, false, false, true, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, true, false, false, false, false, false, true, false, false]);
            let encoded = BinVector::from_bools(&[false, false, true, false, false, false, false, false, true, false, true, false, false, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, true, false, false, false, false, true, true, false, true, false, false, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, false, true, false, true, false, true, false, false, false]);
            let encoded = BinVector::from_bools(&[true, false, false, true, false, true, false, true, false, false, true, false, false, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, false, true, false, true, false, true, false, false, true, false, false, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, false, true, false, true, false, true, true, false, false]);
            let encoded = BinVector::from_bools(&[false, false, false, true, false, true, false, true, true, false, false, false, true, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, false, true, false, true, false, true, true, true, false, false, true, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, true, true, false, true, false, true, false, true, true]);
            let encoded = BinVector::from_bools(&[false, false, true, true, false, true, false, true, false, true, false, true, false, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, true, true, true, true, false, true, false, true, false, true, false, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, true, true, true, true, true, false, true, true, true]);
            let encoded = BinVector::from_bools(&[true, false, true, true, true, true, true, false, true, true, false, true, true, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, true, true, false, true, true, false, true, true, false, true, true, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, true, false, true, true, false, true, false, true, false]);
            let encoded = BinVector::from_bools(&[true, true, true, false, true, true, false, true, false, true, false, false, false, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, true, true, true, true, false, true, false, true, false, false, false, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, false, false, false, true, true, true, false, false, true]);
            let encoded = BinVector::from_bools(&[true, true, false, false, false, true, true, true, false, false, true, true, true, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, false, false, false, true, true, true, false, false, false, true, true, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, false, false, true, true, true, true, true, false, false]);
            let encoded = BinVector::from_bools(&[false, false, false, false, true, true, true, true, true, false, true, false, false, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, false, false, false, true, true, true, true, false, true, false, false, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, true, false, false, true, false, false, false, true, true]);
            let encoded = BinVector::from_bools(&[false, false, true, false, false, true, false, false, false, true, false, true, true, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, true, false, true, true, false, false, false, true, false, true, true, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, true, false, true, true, true, false, true, false, false]);
            let encoded = BinVector::from_bools(&[true, true, true, false, true, true, true, false, true, false, false, false, true, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, true, true, true, true, true, false, true, false, false, false, true, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, false, true, true, true, true, false, true, false, false]);
            let encoded = BinVector::from_bools(&[false, true, false, true, true, true, true, false, true, false, true, false, false, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, false, true, true, true, true, false, true, false, true, false, false, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, false, false, false, true, true, false, false, true, false]);
            let encoded = BinVector::from_bools(&[true, true, false, false, false, true, true, false, false, true, true, false, true, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, false, false, false, true, true, false, false, true, true, false, true, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, false, true, true, false, false, true, false, true, true]);
            let encoded = BinVector::from_bools(&[true, true, false, true, true, false, false, true, false, true, false, true, true, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, false, false, true, false, false, true, false, true, false, true, true, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, false, false, false, true, false, false, false, false, true]);
            let encoded = BinVector::from_bools(&[true, false, false, false, false, true, false, false, false, false, true, true, false, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, false, false, false, true, false, false, true, false, true, true, false, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, false, false, true, true, false, true, true, true, true]);
            let encoded = BinVector::from_bools(&[true, true, false, false, true, true, false, true, true, true, true, true, true, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, false, true, true, true, false, true, true, true, true, true, true, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, true, false, true, true, false, true, true, false, true]);
            let encoded = BinVector::from_bools(&[true, true, true, false, true, true, false, true, true, false, false, true, false, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, true, false, true, true, false, false, true, false, false, true, false, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, true, false, true, true, false, false, false, true, true]);
            let encoded = BinVector::from_bools(&[false, false, true, false, true, true, false, false, false, true, true, true, true, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, true, false, true, true, false, false, false, true, true, true, true, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, true, false, true, true, false, false, false, false, false]);
            let encoded = BinVector::from_bools(&[true, true, true, false, true, true, false, false, false, false, false, false, true, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, true, false, true, true, false, false, false, true, false, false, true, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, false, false, false, false, true, true, true, true, true]);
            let encoded = BinVector::from_bools(&[false, false, false, false, false, false, true, true, true, true, false, true, false, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, false, false, false, false, true, true, true, false, false, true, false, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, false, true, true, true, false, false, true, false, true]);
            let encoded = BinVector::from_bools(&[true, false, false, true, true, true, false, false, true, false, false, true, true, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, false, true, true, true, false, false, true, false, false, false, true, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, true, true, false, true, false, true, true, true, true]);
            let encoded = BinVector::from_bools(&[false, true, true, true, false, true, false, true, true, true, true, true, false, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, true, true, false, false, false, true, true, true, true, true, false, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, false, false, true, false, true, true, true, true, true]);
            let encoded = BinVector::from_bools(&[true, false, false, false, true, false, true, true, true, true, true, true, false, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, false, false, false, false, true, true, true, true, true, true, false, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, false, false, false, false, true, false, false, false, true]);
            let encoded = BinVector::from_bools(&[false, false, false, false, false, false, true, false, false, false, true, true, false, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, false, false, false, false, true, false, false, false, true, true, false, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, true, false, false, false, false, true, true, true, false]);
            let encoded = BinVector::from_bools(&[true, true, true, false, false, false, false, true, true, true, true, false, false, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, true, false, false, false, true, true, true, true, true, false, false, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, true, false, true, true, true, true, false, true, false]);
            let encoded = BinVector::from_bools(&[false, false, true, false, true, true, true, true, false, true, true, false, false, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, true, false, true, true, true, true, false, true, true, false, false, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, true, false, true, true, false, false, true, false, true]);
            let encoded = BinVector::from_bools(&[false, true, true, false, true, true, false, false, true, false, true, true, true, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, true, false, true, true, false, false, false, false, true, true, true, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, true, false, false, false, false, true, false, false, false]);
            let encoded = BinVector::from_bools(&[false, false, true, false, false, false, false, true, false, false, true, false, false, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, true, false, false, false, false, true, false, false, true, false, false, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, true, true, true, false, true, true, true, true, true]);
            let encoded = BinVector::from_bools(&[true, false, true, true, true, false, true, true, true, true, false, true, true, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, true, true, true, true, true, true, true, true, false, true, true, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, false, true, false, true, true, false, true, false, true]);
            let encoded = BinVector::from_bools(&[true, true, false, true, false, true, true, false, true, false, false, true, true, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, false, true, false, true, true, false, true, false, false, true, true, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, false, true, true, false, true, false, false, false, true]);
            let encoded = BinVector::from_bools(&[false, false, false, true, true, false, true, false, false, false, true, true, false, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, false, true, true, false, true, false, false, false, true, true, false, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, false, false, true, false, true, true, false, false, false]);
            let encoded = BinVector::from_bools(&[true, false, false, false, true, false, true, true, false, false, true, false, false, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, false, false, true, false, true, true, false, false, true, false, false, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, true, true, true, false, false, true, true, true, false]);
            let encoded = BinVector::from_bools(&[true, false, true, true, true, false, false, true, true, true, true, false, true, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, true, true, true, false, false, true, true, true, false, false, true, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, true, true, false, false, true, false, true, true, true]);
            let encoded = BinVector::from_bools(&[false, true, true, true, false, false, true, false, true, true, false, true, true, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, true, true, false, false, true, false, true, true, false, false, true, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, true, false, false, true, true, true, false, true, false]);
            let encoded = BinVector::from_bools(&[true, true, true, false, false, true, true, true, false, true, false, false, true, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, false, false, false, true, true, true, false, true, false, false, true, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, true, false, true, false, false, false, false, true, true]);
            let encoded = BinVector::from_bools(&[false, false, true, false, true, false, false, false, false, true, false, true, false, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, true, false, true, false, true, false, false, true, false, true, false, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, false, false, false, false, false, false, false, false, true]);
            let encoded = BinVector::from_bools(&[true, true, false, false, false, false, false, false, false, false, false, true, false, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, false, false, false, false, false, false, false, false, false, true, true, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, false, true, true, false, false, false, false, false, true]);
            let encoded = BinVector::from_bools(&[false, true, false, true, true, false, false, false, false, false, false, true, false, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, false, true, true, false, false, true, false, false, false, true, false, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, false, false, true, false, false, true, true, false, false]);
            let encoded = BinVector::from_bools(&[true, false, false, false, true, false, false, true, true, false, true, false, false, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, false, false, true, false, false, true, true, false, false, false, false, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, true, false, true, true, false, true, false, false, false]);
            let encoded = BinVector::from_bools(&[true, true, true, false, true, true, false, true, false, false, true, false, false, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, true, false, true, true, false, true, false, false, true, false, true, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, true, false, true, true, false, false, false, true, false]);
            let encoded = BinVector::from_bools(&[true, false, true, false, true, true, false, false, false, true, true, false, false, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, true, false, true, true, false, false, true, true, true, false, false, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, true, false, false, true, false, false, false, true, true]);
            let encoded = BinVector::from_bools(&[true, false, true, false, false, true, false, false, false, true, false, true, true, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, true, false, false, true, true, false, false, true, false, true, true, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, true, false, false, false, true, true, true, true, false]);
            let encoded = BinVector::from_bools(&[true, true, true, false, false, false, true, true, true, true, false, false, true, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, true, false, false, false, true, true, true, true, false, false, true, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, false, false, true, true, false, true, false, true, false]);
            let encoded = BinVector::from_bools(&[false, false, false, false, true, true, false, true, false, true, false, false, false, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, false, false, true, true, false, true, false, false, false, false, false, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, false, false, false, false, false, false, false, true, true]);
            let encoded = BinVector::from_bools(&[true, true, false, false, false, false, false, false, false, true, true, true, false, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, false, false, false, false, false, false, false, true, true, true, true, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, false, true, false, false, true, true, true, true, true]);
            let encoded = BinVector::from_bools(&[false, false, false, true, false, false, true, true, true, true, true, true, false, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, false, false, false, false, true, true, true, true, true, true, false, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, false, false, true, false, false, true, true, true, false]);
            let encoded = BinVector::from_bools(&[true, false, false, false, true, false, false, true, true, true, false, false, false, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, false, false, true, false, false, true, true, true, false, false, false, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, false, false, true, false, false, false, true, true, true]);
            let encoded = BinVector::from_bools(&[false, true, false, false, true, false, false, false, true, true, true, true, true, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, false, false, true, false, false, false, true, false, true, true, true, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, true, true, true, false, true, true, false, true, false]);
            let encoded = BinVector::from_bools(&[true, false, true, true, true, false, true, true, false, true, true, false, true, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, true, false, true, false, true, true, false, true, true, false, true, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, true, false, false, true, false, false, true, false, true]);
            let encoded = BinVector::from_bools(&[false, false, true, false, false, true, false, false, true, false, false, true, false, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, true, false, false, true, false, false, true, false, false, true, false, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, false, false, false, false, false, false, true, false, false]);
            let encoded = BinVector::from_bools(&[true, false, false, false, false, false, false, false, true, false, true, false, true, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, false, true, false, false, false, false, true, false, true, false, true, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, false, false, false, true, false, false, true, false, false]);
            let encoded = BinVector::from_bools(&[false, false, false, false, false, true, false, false, true, false, false, false, false, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, false, false, false, true, false, false, false, false, false, false, false, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, true, false, true, true, false, false, false, true, true]);
            let encoded = BinVector::from_bools(&[false, true, true, false, true, true, false, false, false, true, true, true, false, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, true, false, true, true, false, false, false, true, true, true, false, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, true, false, true, false, true, false, false, false, false]);
            let encoded = BinVector::from_bools(&[true, false, true, false, true, false, true, false, false, false, false, false, false, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, true, false, true, false, true, true, false, false, false, false, false, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, false, false, true, false, false, false, false, false, false]);
            let encoded = BinVector::from_bools(&[false, false, false, false, true, false, false, false, false, false, true, false, false, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, false, false, true, false, true, false, false, false, true, false, false, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, false, false, true, false, true, false, true, false, true]);
            let encoded = BinVector::from_bools(&[false, true, false, false, true, false, true, false, true, false, true, true, false, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, false, false, true, false, true, false, true, true, true, true, false, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, false, true, true, false, true, false, true, false, true]);
            let encoded = BinVector::from_bools(&[false, false, false, true, true, false, true, false, true, false, false, true, true, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, true, true, true, false, true, false, true, false, false, true, true, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, true, false, true, false, false, false, true, false, false]);
            let encoded = BinVector::from_bools(&[false, true, true, false, true, false, false, false, true, false, false, false, true, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, false, false, true, false, false, false, true, false, false, false, true, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, true, true, true, false, false, true, true, true, true]);
            let encoded = BinVector::from_bools(&[true, false, true, true, true, false, false, true, true, true, true, true, false, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, true, true, true, false, false, false, true, true, true, true, false, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, false, false, false, false, true, false, false, true, true]);
            let encoded = BinVector::from_bools(&[true, false, false, false, false, false, true, false, false, true, false, true, false, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, false, false, false, false, true, false, true, true, false, true, false, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, true, false, false, false, true, true, false, true, true]);
            let encoded = BinVector::from_bools(&[false, true, true, false, false, false, true, true, false, true, true, true, true, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, false, false, false, false, true, true, false, true, true, true, true, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, true, false, false, true, true, false, true, true, false]);
            let encoded = BinVector::from_bools(&[false, true, true, false, false, true, true, false, true, true, false, false, true, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, true, true, false, true, true, false, true, true, false, false, true, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, false, false, true, false, true, false, false, true, false]);
            let encoded = BinVector::from_bools(&[false, false, false, false, true, false, true, false, false, true, true, false, true, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, false, false, true, false, true, false, false, true, true, false, true, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, true, false, false, false, false, false, false, false, true]);
            let encoded = BinVector::from_bools(&[true, true, true, false, false, false, false, false, false, false, false, true, true, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, true, false, false, true, false, false, false, false, false, true, true, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, true, true, true, true, false, false, false, false, true]);
            let encoded = BinVector::from_bools(&[true, true, true, true, true, true, false, false, false, false, true, true, false, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, true, false, true, true, false, false, false, false, true, true, false, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, true, false, true, false, true, false, true, false, true]);
            let encoded = BinVector::from_bools(&[true, true, true, false, true, false, true, false, true, false, true, true, true, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, true, false, false, false, true, false, true, false, true, true, true, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, true, false, true, false, false, false, true, false, true]);
            let encoded = BinVector::from_bools(&[true, true, true, false, true, false, false, false, true, false, false, true, false, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, true, false, false, false, false, false, true, false, false, true, false, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, false, false, false, false, true, true, true, true, true]);
            let encoded = BinVector::from_bools(&[true, false, false, false, false, false, true, true, true, true, false, true, false, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, false, false, false, false, true, true, true, true, true, true, false, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, false, true, false, true, false, false, true, false, false]);
            let encoded = BinVector::from_bools(&[false, false, false, true, false, true, false, false, true, false, true, false, false, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, false, true, false, false, false, false, true, false, true, false, false, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, false, true, false, false, true, false, true, true, false]);
            let encoded = BinVector::from_bools(&[false, false, false, true, false, false, true, false, true, true, false, false, false, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, false, true, false, false, true, false, true, true, true, false, false, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, true, true, false, true, true, true, true, false, true]);
            let encoded = BinVector::from_bools(&[true, false, true, true, false, true, true, true, true, false, true, true, false, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, true, true, false, true, true, true, true, false, true, true, false, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, false, false, true, false, false, false, false, true, true]);
            let encoded = BinVector::from_bools(&[false, false, false, false, true, false, false, false, false, true, false, true, true, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, false, false, true, false, false, true, false, true, false, true, true, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, false, true, true, false, false, true, true, false, true]);
            let encoded = BinVector::from_bools(&[true, false, false, true, true, false, false, true, true, false, false, true, true, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, false, true, true, false, false, true, true, false, true, true, true, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, true, true, true, true, false, true, true, true, true]);
            let encoded = BinVector::from_bools(&[true, false, true, true, true, true, false, true, true, true, false, true, true, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, true, false, true, true, false, true, true, true, false, true, true, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, true, true, false, false, false, true, false, false, true]);
            let encoded = BinVector::from_bools(&[false, false, true, true, false, false, false, true, false, false, false, true, true, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, true, true, true, false, false, true, false, false, false, true, true, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, false, true, false, true, false, true, true, false, true]);
            let encoded = BinVector::from_bools(&[false, true, false, true, false, true, false, true, true, false, false, true, true, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, false, false, false, true, false, true, true, false, false, true, true, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, true, true, false, false, false, true, false, false, true]);
            let encoded = BinVector::from_bools(&[true, true, true, true, false, false, false, true, false, false, false, true, false, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, true, true, false, false, false, true, false, true, false, true, false, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, false, false, true, true, true, true, true, true, false]);
            let encoded = BinVector::from_bools(&[false, false, false, false, true, true, true, true, true, true, false, false, false, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, false, false, true, true, true, false, true, true, false, false, false, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, true, true, false, true, true, false, true, true, false]);
            let encoded = BinVector::from_bools(&[true, true, true, true, false, true, true, false, true, true, true, false, true, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, true, true, false, true, true, true, true, true, true, false, true, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, true, true, true, true, true, false, false, true, true]);
            let encoded = BinVector::from_bools(&[false, true, true, true, true, true, true, false, false, true, true, true, true, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, true, true, true, true, true, false, false, true, true, false, true, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, true, true, true, false, true, true, false, false, true]);
            let encoded = BinVector::from_bools(&[false, false, true, true, true, false, true, true, false, false, false, true, false, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, true, true, true, false, false, true, false, false, false, true, false, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, false, true, false, false, true, true, false, false, true]);
            let encoded = BinVector::from_bools(&[false, false, false, true, false, false, true, true, false, false, true, true, true, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, false, true, false, false, true, true, false, false, true, true, true, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, false, false, true, false, true, true, false, true, true]);
            let encoded = BinVector::from_bools(&[false, false, false, false, true, false, true, true, false, true, false, true, true, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, false, false, true, true, true, true, false, true, false, true, true, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, false, false, true, true, true, true, true, true, true]);
            let encoded = BinVector::from_bools(&[false, false, false, false, true, true, true, true, true, true, false, true, true, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, false, false, true, true, true, true, true, true, false, true, true, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, false, false, true, true, true, false, false, true, true]);
            let encoded = BinVector::from_bools(&[true, false, false, false, true, true, true, false, false, true, false, true, true, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, false, true, true, true, true, false, false, true, false, true, true, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, true, false, false, false, true, true, false, true, false]);
            let encoded = BinVector::from_bools(&[true, true, true, false, false, false, true, true, false, true, true, false, false, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, false, false, false, false, true, true, false, true, true, false, false, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, false, false, true, false, true, true, false, true, true]);
            let encoded = BinVector::from_bools(&[true, false, false, false, true, false, true, true, false, true, false, true, true, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, false, false, true, false, true, true, false, false, false, true, true, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, false, false, true, false, false, true, true, false, true]);
            let encoded = BinVector::from_bools(&[false, true, false, false, true, false, false, true, true, false, true, true, false, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, false, false, true, true, false, true, true, false, true, true, false, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, false, true, false, false, true, false, false, false, false]);
            let encoded = BinVector::from_bools(&[false, false, false, true, false, false, true, false, false, false, false, false, true, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, false, false, false, false, true, false, false, false, false, false, true, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, false, true, true, false, true, false, false, false, true]);
            let encoded = BinVector::from_bools(&[true, true, false, true, true, false, true, false, false, false, true, true, true, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, false, true, true, false, true, true, false, false, true, true, true, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, true, false, true, true, true, false, true, true, true]);
            let encoded = BinVector::from_bools(&[false, true, true, false, true, true, true, false, true, true, true, true, false, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, true, false, true, true, true, false, true, true, true, true, false, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, true, true, false, true, false, true, false, true, true]);
            let encoded = BinVector::from_bools(&[false, true, true, true, false, true, false, true, false, true, false, true, true, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, true, true, false, false, false, true, false, true, false, true, true, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, true, true, true, false, false, true, true, false, false]);
            let encoded = BinVector::from_bools(&[false, false, true, true, true, false, false, true, true, false, false, false, true, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, true, true, true, false, true, true, true, false, false, false, true, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, true, true, false, true, false, false, true, false, false]);
            let encoded = BinVector::from_bools(&[true, true, true, true, false, true, false, false, true, false, true, false, false, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, true, true, false, true, false, false, true, false, true, false, false, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, true, true, false, true, false, false, false, false, true]);
            let encoded = BinVector::from_bools(&[false, false, true, true, false, true, false, false, false, false, false, true, true, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, true, false, false, true, false, false, false, false, false, true, true, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, true, false, false, false, true, false, true, true, false]);
            let encoded = BinVector::from_bools(&[false, false, true, false, false, false, true, false, true, true, true, false, true, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, true, false, false, false, true, false, true, true, true, false, true, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, false, false, false, false, true, false, true, true, false]);
            let encoded = BinVector::from_bools(&[true, true, false, false, false, false, true, false, true, true, true, false, true, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, false, false, false, false, true, false, true, true, false, false, true, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, false, false, true, false, false, true, true, false, true]);
            let encoded = BinVector::from_bools(&[true, false, false, false, true, false, false, true, true, false, true, true, true, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, false, false, true, false, false, false, true, false, true, true, true, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, false, false, false, false, false, true, false, false, false]);
            let encoded = BinVector::from_bools(&[false, false, false, false, false, false, false, true, false, false, true, false, true, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, false, false, false, false, false, true, false, false, true, false, true, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, false, true, false, false, true, true, false, false, true]);
            let encoded = BinVector::from_bools(&[true, true, false, true, false, false, true, true, false, false, true, true, false, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, false, true, false, false, true, true, false, false, false, true, false, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, false, true, false, true, false, false, true, false, true]);
            let encoded = BinVector::from_bools(&[true, false, false, true, false, true, false, false, true, false, true, true, true, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, false, true, false, true, true, false, true, false, true, true, true, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, true, true, false, false, false, false, false, true, false]);
            let encoded = BinVector::from_bools(&[false, true, true, true, false, false, false, false, false, true, false, false, false, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, true, true, false, false, false, false, false, true, true, false, false, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, false, true, false, true, false, false, false, true, false]);
            let encoded = BinVector::from_bools(&[true, true, false, true, false, true, false, false, false, true, true, false, false, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, false, true, false, true, false, false, false, true, true, false, false, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, true, true, false, true, true, false, true, true, false]);
            let encoded = BinVector::from_bools(&[false, true, true, true, false, true, true, false, true, true, true, false, true, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, true, true, false, true, true, false, true, true, false, false, true, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, true, true, false, true, false, false, false, false, false]);
            let encoded = BinVector::from_bools(&[true, false, true, true, false, true, false, false, false, false, false, false, false, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, true, true, false, true, false, false, false, false, false, false, false, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, false, true, false, true, true, true, true, true, true]);
            let encoded = BinVector::from_bools(&[false, false, false, true, false, true, true, true, true, true, false, true, true, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, false, true, false, true, false, true, true, true, false, true, true, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, false, false, true, true, false, true, false, true, true]);
            let encoded = BinVector::from_bools(&[false, false, false, false, true, true, false, true, false, true, false, true, true, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, false, false, true, true, false, true, false, true, false, true, true, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, false, true, false, false, false, true, false, true, false]);
            let encoded = BinVector::from_bools(&[false, true, false, true, false, false, false, true, false, true, true, false, false, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, true, true, false, false, false, true, false, true, true, false, false, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, false, false, true, false, false, true, false, false, false]);
            let encoded = BinVector::from_bools(&[false, false, false, false, true, false, false, true, false, false, false, false, true, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, false, false, true, false, false, true, false, false, false, false, true, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, false, false, false, true, false, true, false, true, false]);
            let encoded = BinVector::from_bools(&[false, true, false, false, false, true, false, true, false, true, true, false, true, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, false, false, false, false, false, true, false, true, true, false, true, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, true, true, true, true, false, false, true, true, true]);
            let encoded = BinVector::from_bools(&[false, true, true, true, true, true, false, false, true, true, true, true, true, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, true, true, true, true, false, false, false, true, true, true, true, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, true, true, true, true, false, false, true, false, false]);
            let encoded = BinVector::from_bools(&[true, true, true, true, true, true, false, false, true, false, false, false, false, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, true, true, true, true, false, false, true, false, false, true, false, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, false, true, true, false, true, false, true, true, false]);
            let encoded = BinVector::from_bools(&[false, false, false, true, true, false, true, false, true, true, true, false, false, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, false, true, false, false, true, false, true, true, true, false, false, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, false, true, true, false, true, true, false, false, true]);
            let encoded = BinVector::from_bools(&[true, false, false, true, true, false, true, true, false, false, false, true, true, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, false, true, false, false, true, true, false, false, false, true, true, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, true, false, true, false, false, true, false, false, false]);
            let encoded = BinVector::from_bools(&[true, true, true, false, true, false, false, true, false, false, false, false, true, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, true, false, true, true, false, true, false, false, false, false, true, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, false, false, false, true, false, false, false, false, false]);
            let encoded = BinVector::from_bools(&[false, false, false, false, false, true, false, false, false, false, true, false, true, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, false, false, false, true, false, false, true, false, true, false, true, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, true, false, false, true, true, true, false, false, true]);
            let encoded = BinVector::from_bools(&[true, false, true, false, false, true, true, true, false, false, true, true, true, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, true, false, false, true, true, true, false, false, true, true, false, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, true, false, true, false, true, true, false, false, false]);
            let encoded = BinVector::from_bools(&[false, true, true, false, true, false, true, true, false, false, true, false, false, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, true, false, true, false, true, true, false, false, true, true, false, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, true, true, false, true, false, false, true, false, false]);
            let encoded = BinVector::from_bools(&[false, false, true, true, false, true, false, false, true, false, true, false, true, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, true, true, false, true, false, false, true, false, false, false, true, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, true, false, false, true, true, false, false, true, false]);
            let encoded = BinVector::from_bools(&[false, true, true, false, false, true, true, false, false, true, true, false, false, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, true, false, false, true, true, false, true, true, true, false, false, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, true, true, true, true, true, false, false, false, false]);
            let encoded = BinVector::from_bools(&[true, true, true, true, true, true, true, false, false, false, false, false, false, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, true, true, true, true, true, false, true, false, false, false, false, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, true, true, false, false, true, false, true, true, true]);
            let encoded = BinVector::from_bools(&[true, false, true, true, false, false, true, false, true, true, false, true, false, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, true, true, false, false, true, false, true, false, false, true, false, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, false, false, true, false, true, true, false, false, true]);
            let encoded = BinVector::from_bools(&[false, true, false, false, true, false, true, true, false, false, true, true, false, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, false, false, true, false, true, true, false, false, true, true, false, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, false, false, true, false, true, true, true, true, false]);
            let encoded = BinVector::from_bools(&[false, true, false, false, true, false, true, true, true, true, true, false, false, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, false, false, true, false, true, true, true, true, true, false, false, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, true, false, true, false, true, false, false, false, true]);
            let encoded = BinVector::from_bools(&[false, false, true, false, true, false, true, false, false, false, false, true, true, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, true, false, true, false, true, false, false, false, false, false, true, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, true, true, false, false, false, true, false, false, true]);
            let encoded = BinVector::from_bools(&[false, true, true, true, false, false, false, true, false, false, false, true, false, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, true, true, false, false, false, true, false, false, false, true, false, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, false, false, false, false, false, true, true, false, false]);
            let encoded = BinVector::from_bools(&[true, true, false, false, false, false, false, true, true, false, false, false, true, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, false, false, false, false, false, true, true, false, false, false, false, false, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, true, true, true, false, true, true, true, false, true]);
            let encoded = BinVector::from_bools(&[true, true, true, true, true, false, true, true, true, false, true, true, false, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, true, true, true, false, true, true, true, false, true, true, false, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, false, false, false, false, false, false, true, true, false]);
            let encoded = BinVector::from_bools(&[false, true, false, false, false, false, false, false, true, true, false, false, false, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, false, false, false, false, false, false, false, true, false, false, false, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, false, false, false, true, true, true, true, true, true]);
            let encoded = BinVector::from_bools(&[false, true, false, false, false, true, true, true, true, true, true, true, false, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, false, false, false, true, true, true, true, true, true, true, false, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, false, false, true, true, true, true, false, true, true, true]);
            let encoded = BinVector::from_bools(&[true, false, false, true, true, true, true, false, true, true, false, true, false, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, false, false, true, false, true, true, false, true, true, false, true, false, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, false, false, false, true, true, true, true, false, true]);
            let encoded = BinVector::from_bools(&[false, false, false, false, false, true, true, true, true, false, false, true, true, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, false, false, false, false, true, true, true, false, false, true, true, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, false, true, true, true, true, true, false, true, false]);
            let encoded = BinVector::from_bools(&[false, true, false, true, true, true, true, true, false, true, false, false, false, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, false, true, true, true, true, true, false, true, false, false, true, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[true, true, false, false, false, true, false, false, false, true, false]);
            let encoded = BinVector::from_bools(&[true, true, false, false, false, true, false, false, false, true, false, false, false, false, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[true, true, false, false, false, true, false, false, false, true, false, false, false, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, true, true, false, true, false, false, true, false, false]);
            let encoded = BinVector::from_bools(&[false, true, true, true, false, true, false, false, true, false, true, false, false, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, false, true, false, true, false, false, true, false, true, false, false, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, true, false, false, false, true, false, false, false, true]);
            let encoded = BinVector::from_bools(&[false, true, true, false, false, false, true, false, false, false, true, true, false, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, true, false, false, false, false, false, false, false, true, true, false, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, false, false, false, false, true, true, true, false, true]);
            let encoded = BinVector::from_bools(&[false, true, false, false, false, false, true, true, true, false, true, true, true, true, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, false, false, false, false, false, true, true, false, true, true, true, true, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, false, true, true, true, false, false, true, false, true, true]);
            let encoded = BinVector::from_bools(&[false, false, true, true, true, false, false, true, false, true, false, true, true, true, true]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, false, true, true, true, false, false, true, false, false, false, true, true, true, true]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

        {
            let m = BinVector::from_bools(&[false, true, false, false, false, false, false, false, true, true, true]);
            let encoded = BinVector::from_bools(&[false, true, false, false, false, false, false, false, true, true, false, true, true, false, false]);
            assert_eq!(code.encode(m.clone()), encoded);

            {
                let errored = BinVector::from_bools(&[false, true, false, false, true, false, false, false, true, true, false, true, true, false, false]);
                assert_eq!(m.clone(), code.decode_to_message(errored.clone()), "decode to msg failed");
                assert_eq!(encoded.clone(), code.decode_to_code(errored), "decode to code failed");
            }

        }

    }

}
