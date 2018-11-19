extern crate lpn;
extern crate rayon;
extern crate serde;
extern crate serde_json;

use lpn::codes::*;

fn get_code() -> StGenCode<'static> {
    let codes: Vec<&dyn BinaryCode> = vec![
        &HammingCode7_4,
        &HammingCode7_4,
        &HammingCode7_4,
    ];
    StGenCode::new(codes, 3, 100, 3, 2)
}

fn main() {
    let code = get_code();
    println!("Code: [{}, {}]", code.length(), code.dimension());

    // serialize
    println!("{}", serde_json::to_string(&code).unwrap());

}
