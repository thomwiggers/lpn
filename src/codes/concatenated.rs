use codes::BinaryCode;
use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;
use std::cell::UnsafeCell;
use std::ptr;
use std::sync::Mutex;

pub struct ConcatenatedCode<T> {
    codes: Vec<Box<T>>,
    init: Mutex<bool>,
    generator: UnsafeCell<*mut BinMatrix>,
}

impl<'a, T> ConcatenatedCode<T> {
    pub fn new(codes: Vec<Box<T>>) -> ConcatenatedCode<T> {
        ConcatenatedCode {
            codes,
            init: Mutex::new(false),
            generator: UnsafeCell::new(ptr::null_mut()),
        }
    }
}

impl<'a, T: BinaryCode<'a>> BinaryCode<'a> for ConcatenatedCode<T> {
    fn length(&self) -> usize {
        self.codes.iter().fold(0usize, |a, c| a + c.length())
    }

    fn dimension(&self) -> usize {
        self.codes.iter().fold(0usize, |a, c| a + c.dimension())
    }

    fn generator_matrix(&self) -> &'a BinMatrix {
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
                    gen = Box::new(gen.augmented(&BinMatrix::zero(gen.ncols(), code.length())));
                    gen = Box::new(gen.stacked(&BinMatrix::zero(code.dimension(), gen.ncols())));
                }
                unsafe {
                    *(self.generator.get()) = Box::into_raw(gen);
                }
            };
        }
        unsafe { (*(self.generator.get())).as_ref().unwrap() }
    }

    fn parity_check_matrix(&self) -> &'a BinMatrix {
        panic!("Not yet implemented");
    }

    fn decode_to_message(&self, c: &BinVector) -> BinVector {
        let mut decoded = BinVector::with_capacity(self.dimension());
        let mut to_decode = c.clone();
        for code in self.codes.iter() {
            let slice = to_decode.split_off(code.length());
            let decoded_slice = code.decode_to_message(&BinVector::from(slice));
            decoded.extend(decoded_slice.iter());
        }
        decoded
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use codes::hamming::*;
    use m4ri_rust::friendly::BinVector;

    fn get_code() -> ConcatenatedCode<BinaryCode<'static>> {
        ConcatenatedCode::new(vec![Box::new(HammingCode7_4), Box::new(HammingCode3_1)])
    }

}
