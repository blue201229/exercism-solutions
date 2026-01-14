pub fn egg_count(display_value: u32) -> usize {
    let mut egg_count = 0;
    let mut val = display_value;
    while val > 0 {
        egg_count += val % 2;
        val = val /2;
    }
    egg_count as usize
}
