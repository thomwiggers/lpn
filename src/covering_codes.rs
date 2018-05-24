use codes::BinaryCode;
use oracle::LpnOracle;

/// Reduce using the covering codes attack (Guo, Johansson, Lohndal; 2014)
pub fn reduce_covering_codes<'a, T: BinaryCode<'a>>(mut oracle: LpnOracle, code: T) -> LpnOracle {
    assert_eq!(
        oracle.k as usize,
        code.length(),
        "The length of the code does not match the problem size!"
    );

    println!("Decoding queries");
    for mut query in &mut oracle.queries {
        (*query).a = code.decode_to_message(&query.a);
    }
    println!("Note that we decoded the secret!");
    oracle.secret = code.decode_to_message(&oracle.secret);
    debug_assert_eq!(oracle.secret.len(), code.dimension());
    oracle.k = code.dimension() as u32;

    oracle
}
