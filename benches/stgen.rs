#![feature(test)]
extern crate lpn;
extern crate m4ri_rust;
extern crate test;


mod stgen {
    use lpn::codes::*;
    use m4ri_rust::friendly::*;
    use test::Bencher;

    fn get_code() -> StGenCode<'static, 'static> {
        let codes: Vec<&BinaryCode<'static>> = vec![
            &HammingCode7_4,
            &HammingCode7_4,
            &HammingCode7_4,
            &HammingCode7_4,
            &HammingCode7_4,
            &HammingCode7_4,
            &HammingCode7_4,
            &HammingCode7_4,
            &HammingCode7_4,
            &HammingCode7_4,
            &HammingCode7_4,
        ];
        StGenCode::new(codes, 5, 100, 4)
    }

    #[bench]
    fn stgen_encode(b: &mut Bencher) {
        let code = get_code();

        b.iter(|| {
            let i = BinVector::random(code.dimension());
            code.encode(&i)
        });
    }

    #[bench]
    fn decode(b: &mut Bencher) {
        let code = get_code();

        b.iter(|| {
            let i = BinVector::random(code.length());
            code.decode_to_message(&i)
        });
    }

}
