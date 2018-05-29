use codes::BinaryCode;
use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;
use std::cell::UnsafeCell;
use std::ptr;
use std::sync::Mutex;

/// 'Concatenated' Linear Codes
///
/// This struct allows to construct a Linear code from the direct sum
/// of smaller codes.
///
/// It will construct the generator matrix lazily and use the encode and
/// decode mechanism of the 'child' codes.
pub struct ConcatenatedCode<'a, 'c: 'a> {
    codes: Vec<&'a BinaryCode<'c>>,
    init: Mutex<bool>,
    generator: UnsafeCell<*mut BinMatrix>,
}

unsafe impl<'a, 'b> Sync for ConcatenatedCode<'a, 'b> {}

impl<'codes, 'code> ConcatenatedCode<'codes, 'code> {
    pub fn new(codes: Vec<&'codes BinaryCode<'code>>) -> ConcatenatedCode<'codes, 'code> {
        ConcatenatedCode {
            codes,
            init: Mutex::new(false),
            generator: UnsafeCell::new(ptr::null_mut()),
        }
    }
}

impl<'codes, 'code> BinaryCode<'codes> for ConcatenatedCode<'codes, 'code> {
    fn length(&self) -> usize {
        self.codes.iter().fold(0usize, |a, c| a + c.length())
    }

    fn dimension(&self) -> usize {
        self.codes.iter().fold(0usize, |a, c| a + c.dimension())
    }

    fn generator_matrix(&self) -> &'codes BinMatrix {
        debug_assert_ne!(
            self.codes.len(),
            0,
            "We need at least one code for this to work"
        );
        // check if we've initialized the generator
        {
            let initialized: bool = *self.init.lock().unwrap();
            if !initialized {
                let mut gen = Box::new(self.codes[0].generator_matrix().clone());
                for code in self.codes.iter().skip(1) {
                    let corner = (gen.nrows(), gen.ncols());
                    gen = Box::new(gen.augmented(&BinMatrix::zero(gen.nrows(), code.length())));
                    gen = Box::new(gen.stacked(&BinMatrix::zero(code.dimension(), gen.ncols())));
                    gen.set_window(corner.0, corner.1, code.generator_matrix());
                }
                unsafe {
                    *(self.generator.get()) = Box::into_raw(gen);
                }
            };
        }
        unsafe { (*(self.generator.get())).as_ref().unwrap() }
    }

    fn parity_check_matrix(&self) -> &'static BinMatrix {
        panic!("Not yet implemented");
    }

    fn encode(&self, c: &BinVector) -> BinVector {
        let mut encoded = BinVector::with_capacity(self.dimension());
        let mut slice = c.clone();
        for code in self.codes.iter() {
            let next_encode = BinVector::from(slice.split_off(code.dimension()));
            debug_assert_eq!(slice.len(), code.dimension(), "split dimension incorrect");
            encoded.extend_from_binvec(&code.encode(&slice));
            slice = next_encode;
        }
        encoded
    }

    fn decode_to_message(&self, c: &BinVector) -> BinVector {
        let mut decoded = BinVector::with_capacity(self.dimension());
        let mut slice = c.clone();
        for code in self.codes.iter() {
            let next_decode = BinVector::from(slice.split_off(code.length()));
            debug_assert_eq!(slice.len(), code.length(), "Split length incorrect");
            let decoded_slice = code.decode_to_message(&slice);
            decoded.extend_from_binvec(&decoded_slice);
            slice = next_decode;
        }
        decoded
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use codes::hamming::*;
    use m4ri_rust::friendly::BinVector;

    fn get_code() -> ConcatenatedCode<'static, 'static> {
        let codes: Vec<&BinaryCode<'static>> = vec![&HammingCode7_4, &HammingCode3_1];
        ConcatenatedCode::new(codes)
    }

    #[test]
    fn test_concatenated_code() {
        let code = get_code();

        assert_eq!(code.length(), 7 + 3, "Length wrong");
        assert_eq!(code.dimension(), 4 + 1, "Dimension wrong");

        let mut input = BinVector::from_bytes(&[0b10101000]);
        input.truncate(5);
        assert_eq!(
            input,
            BinVector::from_bools(&[true, false, true, false, true])
        );

        let mut encoded = BinVector::from_bytes(&[0b10101011, 0b11000000]);
        encoded.truncate(10);
        assert_eq!(
            *encoded,
            *code.encode(&input),
            "encode(input) doesn't match encoded"
        );

        // idempotent
        assert_eq!(
            input,
            code.decode_to_message(&code.encode(&input)),
            "not idempotent"
        );
    }
}
