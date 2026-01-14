pub fn is_prime(num:u64) -> bool{
    if num < 2 {
        return false;
    }
    for i in 2..=((num as f64).sqrt() as u64) {
        if num % i == 0 {
            return false;
        }
    }
    true
}
pub fn factors(num:u64) -> Vec<u64>{
    let mut res:Vec<u64> = Vec::new();
    if is_prime(num) {
        res.push(num);
        return res
    }
    for i in 2..=((num as f64).sqrt() as u64) {
        if num % i == 0{
            res.push(i);
            res.append(&mut factors(num/i));
            return res;
        }
    }
    res
}
