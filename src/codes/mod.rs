use m4ri_rust::friendly::BinMatrix;
use m4ri_rust::friendly::BinVector;

pub trait BinaryCode<'a> {
    /// Length of the code
    fn length(&self) -> usize;

    /// Dimension of the code
    fn dimension(&self) -> usize;

    /// Generator Matrix
    fn generator_matrix(&self) -> &'a BinMatrix;

    /// Parity check matrix
    fn parity_check_matrix(&self) -> &'a BinMatrix;

    /// Decode a codeword to the codeword space
    fn decode_to_code(&self, c: &BinVector) -> BinVector {
        self.encode(&self.decode_to_message(c))
    }

    /// Decode a codeword to the message space
    fn decode_to_message(&self, c: &BinVector) -> BinVector;

    /// Encode a codeword
    fn encode(&self, c: &BinVector) -> BinVector {
        debug_assert_eq!(
            c.len(),
            self.dimension(),
            "Vector to encode should be of length {}",
            self.dimension()
        );
        let result = c * self.generator_matrix();
        debug_assert_eq!(
            result.len(),
            self.length(),
            "wtf, product should be of length"
        );
        result
    }
}

mod hamming;
pub use self::hamming::*;

mod concatenated;
pub use self::concatenated::*;

mod stgen;
pub use self::stgen::*;
