/// Example of StGen code
extern crate lpn;
extern crate m4ri_rust;
extern crate rayon;

#[cfg(all(feature = "stgen", feature = "hamming"))]
mod themod {
    use lpn::codes::*;

    pub fn get_code() -> StGenCode<'static> {
        let codes: Vec<&dyn BinaryCode> = vec![
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
            &HammingCode7_4,
            &HammingCode7_4,
            &HammingCode7_4,
        ];
        StGenCode::new(codes, 3, 100, 3, 2)
    }
}

#[cfg(all(feature = "stgen", feature = "hamming"))]
fn main() {
    use lpn::codes::*;
    use m4ri_rust::friendly::*;

    let code = themod::get_code();
    println!("Code: [{}, {}]", code.length(), code.dimension());

    //(0..100).into_par_iter().for_each(|_| {
    for _ in 0..10 {
        let i = BinVector::random(code.length());
        code.decode_to_message(&i).unwrap();
    }
    //});
}

#[cfg(not(all(feature = "stgen", feature = "hamming")))]
fn main() {
    println!("Disabled necessary feature, example won't work");
}
