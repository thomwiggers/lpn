pub const fn num_bits<T>() -> usize { std::mem::size_of::<T>() * 8 }

pub fn log_2(x: usize) -> u32 {
    assert!(x > 0);
    num_bits::<usize>() as u32 - x.leading_zeros() - 1
}