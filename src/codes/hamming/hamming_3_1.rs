use codes::BinaryCode;
use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;


pub struct HammingCode3_1;

static SYNDROME: [[bool; 1]; 8] = [
     [false], // (0)
     [false], // (0)
     [false], // (0)
     [true], // (1)
     [false], // (0)
     [true], // (1)
     [true], // (1)
     [true], // (1)
];

static ENCODE: [[bool; 3]; 2] = [
       [false, false, false], // (0, 0, 0)
       [true, true, true], // (1, 1, 1)
];


lazy_static! {
    static ref GENERATOR: BinMatrix = BinMatrix::new(vec![
      BinVector::from_bools(&[true, true, true]),

    ]);
}



impl BinaryCode for HammingCode3_1 {
    fn length() -> usize {
        3
    }

    fn dimension() -> usize {
        1
    }

    fn generator_matrix(&self) -> &'static BinMatrix {
        &GENERATOR
    }

    fn decode_to_message(&self, c: BinVector) -> BinVector {
        debug_assert_eq!(c.len(), Self::length());
        BinVector::from_bools(&SYNDROME[c.as_u32() as usize])
    }

    /// Encode using lookup table
    fn encode(&self, c: BinVector) -> BinVector {
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

        let vec = code.decode_to_codeword(BinVector::from_elem(3, false));
        assert_eq!(vec, BinVector::from_elem(3, false));
    }

}
