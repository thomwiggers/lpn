use codes::BinaryCode;
use m4ri_rust::friendly::*;

pub struct IdentityCode {
    k: usize,
    generator: BinMatrix,
}

impl IdentityCode {
    pub fn new(k: usize) -> IdentityCode {
        IdentityCode {
            k,
            generator: BinMatrix::identity(k),
        }
    }
}

impl<'a, 'c: 'a> BinaryCode<'a> for &'c IdentityCode {
    fn length(&self) -> usize {
        self.k
    }

    fn dimension(&self) -> usize {
        self.k
    }

    fn generator_matrix(&self) -> &'a BinMatrix {
        &self.generator
    }


    fn parity_check_matrix(&self) -> &'a BinMatrix {
        panic!("Doesn't have one");
    }


    fn decode_to_code(&self, c: &BinVector) -> BinVector {
        c.clone()
    }

    fn decode_to_message(&self, c: &BinVector) -> BinVector {
        c.clone()
    }

    fn bias(&self, _delta: f64) -> f64 {
        1f64
    }
}
