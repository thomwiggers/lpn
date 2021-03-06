extern crate itertools;
/// Tries to find new [189,64] StGen codes
extern crate lpn;
extern crate rayon;

#[cfg(all(feature = "stgen", feature = "hamming", feature = "max_k_191"))]
mod program {
    use lpn::codes::*;

    use std::time;

    pub fn main() {
        let idcode1 = IdentityCode::new(1);
        let hamcode = &HammingCode7_4;
        let repcode3 = RepetitionCode::new(3);
        let repcode4 = RepetitionCode::new(4);
        let repcode5 = RepetitionCode::new(5);
        let mut subcodes: Vec<&dyn BinaryCode> = Vec::with_capacity(64);
        subcodes.push(&idcode1);
        subcodes.push(hamcode);
        subcodes.push(hamcode);
        subcodes.push(&repcode4);
        (0..50).for_each(|_| {
            subcodes.push(&repcode3);
        });
        (0..4).for_each(|_| {
            subcodes.push(&repcode5);
        });
        let concatenated = ConcatenatedCode::new(subcodes.clone());
        assert_eq!(concatenated.length(), 189);
        assert_eq!(concatenated.dimension(), 64);

        let delta = 1.0 - 2.0 * 1.0 / 8.0;
        println!("Bias of concatenated code: {:e}", concatenated.bias(delta));

        let initial_weight_range = 1..4;
        let l_max_range = (200..=1000usize).step_by(200);
        let weight_limit_range = initial_weight_range.start..5;
        let weight_increase_range = 1..3;

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
            let start = time::Instant::now();
            let bias = stgen.bias(delta);
            let duration = time::Instant::now() - start;
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
}

#[cfg(all(feature = "stgen", feature = "hamming"))]
fn main() {
    program::main()
}

#[cfg(not(all(feature = "stgen", feature = "hamming")))]
fn main() {
    println!("Disabled necessary feature, example won't work");
}
