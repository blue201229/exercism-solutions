
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // (1..limit).fold( 0, |acc, x| {
    //     for factor in factors{
    //         if *factor == 0 {continue;}
    //         if x % *factor == 0 { return acc + x; }
    //     }
    //     acc
    // })
    
    (1..limit).filter(|n| factors.iter().any(|&factor| factor != 0 && n % factor == 0)).sum()
    
}