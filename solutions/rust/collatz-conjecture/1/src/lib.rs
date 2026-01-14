pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut val = n;
    let mut count = 0;
    while val > 1 {
        val = if val % 2 == 0 { val / 2} else { val * 3 + 1};
        count += 1;
    }
    Some(count)
}
