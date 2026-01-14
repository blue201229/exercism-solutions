pub fn is_prime(num: u32) -> bool {
    if num < 2 {
        return false;
    }
    for i in 2..=((num as f64).sqrt() as u32) {
        if num % i == 0 {
            return false;
        }
    }
    true
}
pub fn nth(n: u32) -> u32 {
    (2..).filter(|&x| is_prime(x)).nth((n) as usize).unwrap()
}
