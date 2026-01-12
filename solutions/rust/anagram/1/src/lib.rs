use std::collections::HashSet;
use std::collections::HashMap;
fn is_anagram(word1:&str, word2:&str) -> bool {
    let word1_lower = word1.to_lowercase();
    let word2_lower = word2.to_lowercase();
    if word1_lower == word2_lower || word1_lower.len() != word2_lower.len() {
        return false;
    }
    let mut char_counter:HashMap<char, i32> = HashMap::new();
    for c in word1_lower.chars() {
        *char_counter.entry(c).or_insert(0) += 1;
    }
    
    for c in word2_lower.chars() {
        let count = char_counter.entry(c).or_insert(0);
        if *count == 0 {
            return false; 
        }
        *count -= 1;
        if *count < 0
        {
            return false;
        }
    }

    return true;
}
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    
    let mut _anagrams:HashSet<&'a str> = HashSet::new();
    for &pos_ana in possible_anagrams {
        if is_anagram(word, pos_ana) {
            _anagrams.insert(pos_ana);
        }        
    }
    _anagrams
}