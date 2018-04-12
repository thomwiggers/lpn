use m4ri_rust::friendly::BinVector;
use m4ri_rust::friendly::BinMatrix;

pub trait BinaryCode {
    fn generator_matrix(&self) -> BinMatrix;

    fn decode_to_code(&self, c: BinVector) -> BinVector {
        self.decode_to_message(c) * self.generator_matrix()
    }

    fn decode_to_message(&self, c: BinVector) -> BinVector;

}

pub mod hamming;
mod syndromes;
