use crate::codes::BinaryCode;
use m4ri_rust::friendly::*;
use std::cmp;

/// $[k, k]$ Identity code
#[derive(Debug, Clone, Serialize)]
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
    /// Create a new instance of a $[k, k]$ code.
    pub fn new(k: usize) -> IdentityCode {
        IdentityCode {
            k,
            generator: BinMatrix::identity(k),
        }
    }
}

impl BinaryCode for IdentityCode {
    fn name(&self) -> String {
        format!("[{k}, {k}] trivial linear code", k = self.k)
    }

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

    fn decode_to_code(&self, c: &BinVector) -> Result<BinVector, &str> {
        Ok(c.clone())
    }

    fn decode_to_message(&self, c: &BinVector) -> Result<BinVector, &str> {
        Ok(c.clone())
    }

    fn bias(&self, _delta: f64) -> f64 {
        1f64
    }
}
