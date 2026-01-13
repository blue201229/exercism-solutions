
pub fn is_armstrong_number(num: u32) -> bool {
    let mut sum = 0;
    if num == 0 { 
        return true;
    }
    let counter= num.ilog10() + 1;
    let mut n = num;
    loop {
        let remain:u32 = n % 10;
        
        n /= 10;
        sum += remain.pow(counter );
       
        if n < 1 {
            break;
        }      
    }
    sum == num
}
