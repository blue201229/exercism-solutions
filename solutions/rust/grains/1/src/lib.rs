pub fn square(s: u32) -> u64 {
    if s > 64 {
        panic!("The value {} is greater than the allowed maximum of 64", s);
    }
    1u64 << (s - 1)
}

pub fn total() -> u64 {
    (1..=64).fold(0,|acc, x| acc + square(x))
}
