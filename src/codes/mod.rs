use m4ri_rust::friendly::BinVector;
use m4ri_rust::friendly::BinMatrix;

pub trait BinaryCode {

    /// Length of the code
    fn length() -> usize;

    /// Dimension of the code
    fn dimension() -> usize;

    /// Generator Matrix
    fn generator_matrix(&self) -> &'static BinMatrix;

    /// Decode a codeword to the codeword space
    fn decode_to_codeword(&self, c: BinVector) -> BinVector {
        self.encode(self.decode_to_message(c))
    }

    /// Decode a codeword to the message space
    fn decode_to_message(&self, c: BinVector) -> BinVector;

    /// Encode a codeword
    fn encode(&self, c: BinVector) -> BinVector {
        debug_assert_eq!(c.len(), Self::length());
        &c * self.generator_matrix()
    }

}

pub mod hamming;
mod syndromes;
