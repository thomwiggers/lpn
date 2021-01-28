/// This file computes the properties of StGen codes based on the
/// code used by Bogos and Vaudenay to attack ``LPN_512,1/8``
extern crate lpn;
extern crate itertools;
extern crate rayon;

#[cfg(all(feature = "bogosrnd", feature="stgen"))]
fn main() {
    use lpn::codes::*;
    use std::time::Instant;

    let subcodes: Vec<&dyn BinaryCode> = vec![
        &BogosrndCode18_6,
        &BogosrndCode19_6,
        &BogosrndCode19_6,
        &BogosrndCode19_6,
        &BogosrndCode19_6,
        &BogosrndCode19_6,
        &BogosrndCode19_7,
        &BogosrndCode19_7,
        &BogosrndCode19_7,
        &BogosrndCode19_7,
    ];
    let concatenated = ConcatenatedCode::new(subcodes.clone());

    println!(
        "Bias of concatenated code: {:e}",
        concatenated.bias(1.0 - 2.0 * 1.0 / 8.0)
    );

    let initial_weight_range = 8..10;
    let l_max_range = (600..=1000usize).step_by(200);
    let weight_limit_range = (initial_weight_range.start + 1)..15;
    let weight_increase_range = 5..10;

    iproduct!(
        initial_weight_range,
        l_max_range,
        weight_limit_range,
        weight_increase_range
    )
    .map(|(w0, l_max, wb, w_inc)| StGenCode::new(subcodes.clone(), w0, l_max, wb, w_inc))
    .collect::<Vec<StGenCode>>()
    .into_iter()
    .for_each(|stgen| {
        let start = Instant::now();
        let bias = stgen.bias(1.0 - 2.0 * 1.0 / 8.0);
        let duration = Instant::now() - start;
        println!(
            "Bias of StGen code ({}, {}, {}, {}): {:e} in {:4.4} s",
            stgen.w0(),
            stgen.l_max(),
            stgen.wb(),
            stgen.w_inc(),
            bias,
            duration.as_secs_f64(),
        );
    });
}

#[cfg(not(all(feature = "bogosrnd", feature="stgen")))]
fn main() {
    println!("Disabled necessary feature, example won't work");
}
