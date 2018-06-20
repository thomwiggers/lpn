use codes::BinaryCode;
use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;
use std::sync::{Once,ONCE_INIT};
use std::boxed::Box;

pub struct HammingCode3_1;

static INIT: Once = ONCE_INIT;
static mut GENERATOR_MATRIX: *const BinMatrix = 0 as *const BinMatrix;
static mut PARITY_MATRIX: *const BinMatrix = 0 as *const BinMatrix;

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
        }
    });
}

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

    fn decode_to_message(&self, c: &BinVector) -> BinVector {
        debug_assert_eq!(c.len(), self.length());
        BinVector::from_bools(&SYNDROME[c.as_u32() as usize])
    }

    /// Encode using lookup table
    fn encode(&self, c: &BinVector) -> BinVector {
        debug_assert_eq!(c.len(), self.dimension());
        BinVector::from_bools(&ENCODE[c.as_u32() as usize])
    }

    // for hamming codes
    fn bias(&self, delta: f64) -> f64 {
        (1f64 + (3 as f64) * delta_s) / ((3 + 1) as f64)
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

}
