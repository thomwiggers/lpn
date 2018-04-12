use codes::BinaryCode;
use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

pub struct HammingCode7_4;

impl BinaryCode for HammingCode7_4 {

    /// FIXME
    fn generator_matrix(&self) -> BinMatrix {
        BinMatrix::identity(4).augmented(&BinMatrix::new(vec![
            BinVector::from_bools(&[true, true, false]),
            BinVector::from_bools(&[true, false, true]),
            BinVector::from_bools(&[false, true, true]),
            BinVector::from_bools(&[true, true, true]),
        ]))
    }

    /// FIXME
    fn decode_to_message(&self, mut c: BinVector) -> BinVector {
        c.truncate(4);
        c
    }
}

fn parity_check_matrix() {
    // all non-zero, length r (3) columns each used exactly once.
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
    }

}
