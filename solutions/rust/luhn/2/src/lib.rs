/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.chars().any(|c| !c.is_ascii_digit() && c != ' ') {
        return false;
    }
    let digits = code.chars().filter(|c| c.is_ascii_digit());
    if digits.clone().count() <= 1 {
    return false;
    }
        
    digits.rev()
        .enumerate()
        .map(|(i, c)| {
            let mut n = c.to_digit(10).unwrap();
            if i % 2 == 1 {
                n *= 2;
                if n > 9 {
                    n -= 9;
                }
            }
            n
        }).sum::<u32>() % 10 == 0
}
