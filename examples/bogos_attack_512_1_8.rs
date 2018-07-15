#![feature(iterator_step_by)]
extern crate lpn;
extern crate time;
#[macro_use]
extern crate itertools;
extern crate rayon;

use lpn::codes::*;

fn main() {
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

    let initial_weight_range = 6..10;
    let l_max_range = (400..2000usize).into_iter().step_by(200);
    let weight_limit_range = initial_weight_range.start..15;
    let weight_increase_range = 4..10;

    iproduct!(initial_weight_range, l_max_range, weight_limit_range, weight_increase_range)
        .map(|(w0, l_max, wb, w_inc)| StGenCode::new(subcodes.clone(), w0, l_max, wb, w_inc))
        .collect::<Vec<StGenCode>>()
        .into_iter()
        .for_each(|stgen| {
            let start = time::precise_time_s();
            let bias = stgen.bias(1.0 - 2.0 * 1.0 / 8.0);
            let duration = time::precise_time_s() - start;
            println!(
                "Bias of StGen code ({}, {}, {}, {}): {:e} in {:4.4} s",
                stgen.w0(),
                stgen.l_max(),
                stgen.wb(),
                stgen.w_inc(),
                bias,
                duration,
            );
        });
}
