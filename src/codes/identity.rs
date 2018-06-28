use codes::BinaryCode;
use m4ri_rust::friendly::*;
use std::cmp;

#[derive(Debug, Clone)]
pub struct IdentityCode {
    k: usize,
    generator: BinMatrix,
}

impl cmp::PartialEq for IdentityCode {
    fn eq(&self, other: &IdentityCode) -> bool {
        self.k == other.k
    }
}

impl cmp::Eq for IdentityCode {}

impl IdentityCode {
    pub fn new(k: usize) -> IdentityCode {
        IdentityCode {
            k,
            generator: BinMatrix::identity(k),
        }
    }
}

impl BinaryCode for IdentityCode {
    fn length(&self) -> usize {
        self.k
    }

    fn dimension(&self) -> usize {
        self.k
    }

    fn generator_matrix(&self) -> &BinMatrix {
        &self.generator
    }

    fn parity_check_matrix(&self) -> &BinMatrix {
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
