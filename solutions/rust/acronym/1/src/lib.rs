pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_whitespace() || c == '-') 
        .filter(|word| !word.is_empty())
        .flat_map(|word| {
            let cleaned: String = word.chars().filter(|c| c.is_alphabetic()).collect();
            let is_all_caps = cleaned.chars().all(|c| c.is_uppercase());
            
            word.chars()
                .filter(|c| c.is_alphabetic())
                .enumerate()
                .filter(move |&(i, c)| i == 0 || (c.is_uppercase() && !is_all_caps)) 
                .map(|(_, c)| c.to_ascii_uppercase()) 
        })
        .collect()
}