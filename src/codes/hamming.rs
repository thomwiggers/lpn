use codes::BinaryCode;
use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

use codes::syndromes::hamming_7_4::SYNDROME;

pub struct HammingCode7_4;

lazy_static! {
    static ref GENERATOR_7_4: BinMatrix = BinMatrix::identity(4).augmented(&BinMatrix::new(vec![
        BinVector::from_bools(&[true, true, false]),
        BinVector::from_bools(&[true, false, true]),
        BinVector::from_bools(&[false, true, true]),
        BinVector::from_bools(&[true, true, true]),
    ]));
}



impl BinaryCode for HammingCode7_4 {
    fn length() -> usize {
        7
    }

    fn dimension() -> usize {
        4
    }

    /// FIXME store this somewhere
    fn generator_matrix(&self) -> &'static BinMatrix {
        &GENERATOR_7_4
    }

    fn decode_to_message(&self, c: BinVector) -> BinVector {
        debug_assert_eq!(c.len(), Self::length());
        BinVector::from_bools(&SYNDROME[c.as_u32() as usize])
    }

    // FIXME
    fn encode(&self, _c: BinVector) -> BinVector {
        panic!("Not yet implemented");
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
