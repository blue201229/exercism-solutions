
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit).fold( 0, |acc, x| {
        for factor in factors{
            if *factor == 0 {continue;}
            if x % *factor == 0 { return acc + x; }
        }
        acc
    })
    // let mut sum = 0;
    // for factor in factors { 
    //     sum += (factor..limit).fold(0, |acc, x| {
    //         if x % factor == 0 {
    //             acc + x
    //         } else {
    //             acc
    //         }
    //     });
    // }
    // sum
}