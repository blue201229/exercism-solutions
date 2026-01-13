/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {


    code.chars().rev().filter(|c|!c.is_whitespace())
        .try_fold((0,0),|(sum,count),val|{
            val.to_digit(10)
                .map(|num|if count%2==1{num*2}else { num })
                .map(|num|if num>9{num-9}else { num })
                .map(|num|(sum+num,count+1))
        }).map_or(false,|(sum,count)|sum%10==0 && count>1)

    // if code.chars().any(|c| !c.is_digit(10) && c != ' ') {
    //     return false;
    // }
    // let digits = code.chars().filter(|c| c.is_digit(10));

    // if digits.count() <= 1 {
    //     return false;
    // }
        
    // digits.rev()
    //     .enumerate()
    //     .map(|(i, c)| {
    //         let mut n = c.to_digit(10).unwrap();
    //         if i % 2 == 0 {
    //             n *= 2;
    //             if n > 9 {
    //                 n -= 9;
    //             }
    //         }
    //         n
    //     }).sum::<u32>() % 10 == 0


        
}

