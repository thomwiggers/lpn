use codes::BinaryCode;
use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;


pub struct HammingCode7_4;

lazy_static! {
    static ref GENERATOR: BinMatrix = BinMatrix::new(vec![
      BinVector::from_bools(&[true, false, false, false, false, true, true]),
      BinVector::from_bools(&[false, true, false, false, true, false, true]),
      BinVector::from_bools(&[false, false, true, false, true, true, false]),
      BinVector::from_bools(&[false, false, false, true, true, true, true]),

    ]);

    static SYNDROME: [[bool; 4]; 128] = [
     [false, false, false, false], // (0, 0, 0, 0)
     [false, false, false, false], // (0, 0, 0, 0)
     [false, false, false, false], // (0, 0, 0, 0)
     [true, true, true, false], // (1, 1, 1, 0)
     [false, false, false, false], // (0, 0, 0, 0)
     [true, true, true, false], // (1, 1, 1, 0)
     [true, true, true, false], // (1, 1, 1, 0)
     [true, true, true, false], // (1, 1, 1, 0)
     [false, false, false, false], // (0, 0, 0, 0)
     [true, false, false, true], // (1, 0, 0, 1)
     [false, true, false, true], // (0, 1, 0, 1)
     [true, true, false, true], // (1, 1, 0, 1)
     [false, false, true, true], // (0, 0, 1, 1)
     [true, false, true, true], // (1, 0, 1, 1)
     [false, true, true, true], // (0, 1, 1, 1)
     [true, true, true, false], // (1, 1, 1, 0)
     [false, false, false, false], // (0, 0, 0, 0)
     [true, false, false, true], // (1, 0, 0, 1)
     [false, true, false, false], // (0, 1, 0, 0)
     [true, true, false, false], // (1, 1, 0, 0)
     [false, false, true, false], // (0, 0, 1, 0)
     [true, false, true, false], // (1, 0, 1, 0)
     [false, true, true, true], // (0, 1, 1, 1)
     [true, true, true, false], // (1, 1, 1, 0)
     [true, false, false, true], // (1, 0, 0, 1)
     [true, false, false, true], // (1, 0, 0, 1)
     [false, true, true, true], // (0, 1, 1, 1)
     [true, false, false, true], // (1, 0, 0, 1)
     [false, true, true, true], // (0, 1, 1, 1)
     [true, false, false, true], // (1, 0, 0, 1)
     [false, true, true, true], // (0, 1, 1, 1)
     [false, true, true, true], // (0, 1, 1, 1)
     [false, false, false, false], // (0, 0, 0, 0)
     [true, false, false, false], // (1, 0, 0, 0)
     [false, true, false, true], // (0, 1, 0, 1)
     [true, true, false, false], // (1, 1, 0, 0)
     [false, false, true, false], // (0, 0, 1, 0)
     [true, false, true, true], // (1, 0, 1, 1)
     [false, true, true, false], // (0, 1, 1, 0)
     [true, true, true, false], // (1, 1, 1, 0)
     [false, true, false, true], // (0, 1, 0, 1)
     [true, false, true, true], // (1, 0, 1, 1)
     [false, true, false, true], // (0, 1, 0, 1)
     [false, true, false, true], // (0, 1, 0, 1)
     [true, false, true, true], // (1, 0, 1, 1)
     [true, false, true, true], // (1, 0, 1, 1)
     [false, true, false, true], // (0, 1, 0, 1)
     [true, false, true, true], // (1, 0, 1, 1)
     [false, false, true, false], // (0, 0, 1, 0)
     [true, true, false, false], // (1, 1, 0, 0)
     [true, true, false, false], // (1, 1, 0, 0)
     [true, true, false, false], // (1, 1, 0, 0)
     [false, false, true, false], // (0, 0, 1, 0)
     [false, false, true, false], // (0, 0, 1, 0)
     [false, false, true, false], // (0, 0, 1, 0)
     [true, true, false, false], // (1, 1, 0, 0)
     [false, false, false, true], // (0, 0, 0, 1)
     [true, false, false, true], // (1, 0, 0, 1)
     [false, true, false, true], // (0, 1, 0, 1)
     [true, true, false, false], // (1, 1, 0, 0)
     [false, false, true, false], // (0, 0, 1, 0)
     [true, false, true, true], // (1, 0, 1, 1)
     [false, true, true, true], // (0, 1, 1, 1)
     [true, true, true, true], // (1, 1, 1, 1)
     [false, false, false, false], // (0, 0, 0, 0)
     [true, false, false, false], // (1, 0, 0, 0)
     [false, true, false, false], // (0, 1, 0, 0)
     [true, true, false, true], // (1, 1, 0, 1)
     [false, false, true, true], // (0, 0, 1, 1)
     [true, false, true, false], // (1, 0, 1, 0)
     [false, true, true, false], // (0, 1, 1, 0)
     [true, true, true, false], // (1, 1, 1, 0)
     [false, false, true, true], // (0, 0, 1, 1)
     [true, true, false, true], // (1, 1, 0, 1)
     [true, true, false, true], // (1, 1, 0, 1)
     [true, true, false, true], // (1, 1, 0, 1)
     [false, false, true, true], // (0, 0, 1, 1)
     [false, false, true, true], // (0, 0, 1, 1)
     [false, false, true, true], // (0, 0, 1, 1)
     [true, true, false, true], // (1, 1, 0, 1)
     [false, true, false, false], // (0, 1, 0, 0)
     [true, false, true, false], // (1, 0, 1, 0)
     [false, true, false, false], // (0, 1, 0, 0)
     [false, true, false, false], // (0, 1, 0, 0)
     [true, false, true, false], // (1, 0, 1, 0)
     [true, false, true, false], // (1, 0, 1, 0)
     [false, true, false, false], // (0, 1, 0, 0)
     [true, false, true, false], // (1, 0, 1, 0)
     [false, false, false, true], // (0, 0, 0, 1)
     [true, false, false, true], // (1, 0, 0, 1)
     [false, true, false, false], // (0, 1, 0, 0)
     [true, true, false, true], // (1, 1, 0, 1)
     [false, false, true, true], // (0, 0, 1, 1)
     [true, false, true, false], // (1, 0, 1, 0)
     [false, true, true, true], // (0, 1, 1, 1)
     [true, true, true, true], // (1, 1, 1, 1)
     [true, false, false, false], // (1, 0, 0, 0)
     [true, false, false, false], // (1, 0, 0, 0)
     [false, true, true, false], // (0, 1, 1, 0)
     [true, false, false, false], // (1, 0, 0, 0)
     [false, true, true, false], // (0, 1, 1, 0)
     [true, false, false, false], // (1, 0, 0, 0)
     [false, true, true, false], // (0, 1, 1, 0)
     [false, true, true, false], // (0, 1, 1, 0)
     [false, false, false, true], // (0, 0, 0, 1)
     [true, false, false, false], // (1, 0, 0, 0)
     [false, true, false, true], // (0, 1, 0, 1)
     [true, true, false, true], // (1, 1, 0, 1)
     [false, false, true, true], // (0, 0, 1, 1)
     [true, false, true, true], // (1, 0, 1, 1)
     [false, true, true, false], // (0, 1, 1, 0)
     [true, true, true, true], // (1, 1, 1, 1)
     [false, false, false, true], // (0, 0, 0, 1)
     [true, false, false, false], // (1, 0, 0, 0)
     [false, true, false, false], // (0, 1, 0, 0)
     [true, true, false, false], // (1, 1, 0, 0)
     [false, false, true, false], // (0, 0, 1, 0)
     [true, false, true, false], // (1, 0, 1, 0)
     [false, true, true, false], // (0, 1, 1, 0)
     [true, true, true, true], // (1, 1, 1, 1)
     [false, false, false, true], // (0, 0, 0, 1)
     [false, false, false, true], // (0, 0, 0, 1)
     [false, false, false, true], // (0, 0, 0, 1)
     [true, true, true, true], // (1, 1, 1, 1)
     [false, false, false, true], // (0, 0, 0, 1)
     [true, true, true, true], // (1, 1, 1, 1)
     [true, true, true, true], // (1, 1, 1, 1)
     [true, true, true, true], // (1, 1, 1, 1)
];

    static ENCODE: [[bool; 4; 16] = [
       [false, false, false, false, false, false, false], // (0, 0, 0, 0, 0, 0, 0)
       [true, false, false, false, false, true, true], // (1, 0, 0, 0, 0, 1, 1)
       [false, true, false, false, true, false, true], // (0, 1, 0, 0, 1, 0, 1)
       [true, true, false, false, true, true, false], // (1, 1, 0, 0, 1, 1, 0)
       [false, false, true, false, true, true, false], // (0, 0, 1, 0, 1, 1, 0)
       [true, false, true, false, true, false, true], // (1, 0, 1, 0, 1, 0, 1)
       [false, true, true, false, false, true, true], // (0, 1, 1, 0, 0, 1, 1)
       [true, true, true, false, false, false, false], // (1, 1, 1, 0, 0, 0, 0)
       [false, false, false, true, true, true, true], // (0, 0, 0, 1, 1, 1, 1)
       [true, false, false, true, true, false, false], // (1, 0, 0, 1, 1, 0, 0)
       [false, true, false, true, false, true, false], // (0, 1, 0, 1, 0, 1, 0)
       [true, true, false, true, false, false, true], // (1, 1, 0, 1, 0, 0, 1)
       [false, false, true, true, false, false, true], // (0, 0, 1, 1, 0, 0, 1)
       [true, false, true, true, false, true, false], // (1, 0, 1, 1, 0, 1, 0)
       [false, true, true, true, true, false, false], // (0, 1, 1, 1, 1, 0, 0)
       [true, true, true, true, true, true, true], // (1, 1, 1, 1, 1, 1, 1)
];
}



impl BinaryCode for HammingCode7_4 {
    fn length() -> usize {
        7
    }

    fn dimension() -> usize {
        4
    }

    fn generator_matrix(&self) -> &'static BinMatrix {
        &GENERATOR
    }

    fn decode_to_message(&self, c: BinVector) -> BinVector {
        debug_assert_eq!(c.len(), Self::length());
        BinVector::from_bools(&SYNDROME[c.as_u32() as usize])
    }

    /// Encode using lookup table
    fn encode(&self, _c: BinVector) -> BinVector {
        debug_assert_eq!(c.len(), Self::dimension());
        BinVector::from_bools(&ENCODE[c.as_u32() as usize])
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

        let codeword = code.decode_to_message(BinVector::from_elem(7, true));
        assert_eq!(codeword, BinVector::from_elem(4, true));

        let mut vec = BinVector::from_elem(7, true);
        vec.set(0, false);
        let codeword = code.decode_to_message(vec);
        assert_eq!(codeword, BinVector::from_elem(4, true));

        let vec = code.decode_to_codeword(BinVector::from_elem(7, false));
        assert_eq!(codeword, BinVector::from_elem(7, false));
    }

}
