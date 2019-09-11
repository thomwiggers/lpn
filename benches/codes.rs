#![feature(test)]
extern crate lpn;
extern crate m4ri_rust;
extern crate test;

macro_rules! bench_code {
    ($name:ident, $code:expr) => {
        mod $name {
            use crate::test::Bencher;
            use lpn::codes::*;
            use m4ri_rust::friendly::*;

            #[bench]
            fn random_vector_dimension(b: &mut Bencher) {
                let code = $code;

                b.iter(|| BinVector::random(code.dimension()));
            }

            #[bench]
            fn random_vector_length(b: &mut Bencher) {
                let code = $code;

                b.iter(|| BinVector::random(code.length()));
            }

            #[bench]
            fn encode(b: &mut Bencher) {
                let code = $code;

                b.iter(|| {
                    let i = BinVector::random(code.dimension());
                    code.encode(&i)
                });
            }

            #[bench]
            fn decode_to_code(b: &mut Bencher) {
                let code = $code;

                b.iter(|| {
                    let i = BinVector::random(code.length());
                    code.decode_to_code(&i)
                });
            }

            #[bench]
            fn decode_to_message(b: &mut Bencher) {
                let code = $code;

                b.iter(|| {
                    let i = BinVector::random(code.length());
                    code.decode_to_message(&i)
                });
            }
        }
    };
}

bench_code!(hamming_3_1, HammingCode3_1);
bench_code!(hamming_7_4, HammingCode7_4);
bench_code!(hamming_15_11, HammingCode15_11);
bench_code!(hamming_31_26, HammingCode31_26);
bench_code!(hamming_63_57, HammingCode63_57);
bench_code!(hamming_127_120, HammingCode127_120);
bench_code!(golay_23_12, GolayCode23_12);
bench_code!(golay_24_12, GolayCode24_12);
