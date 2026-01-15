use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // Parse the equation - handle both = and ==
    let input_normalized = input.replace("==", "=");
    let parts: Vec<&str> = input_normalized.split('=').map(|s| s.trim()).collect();
    if parts.len() != 2 {
        return None;
    }

    let left_side = parts[0];
    let right_side = parts[1];

    // Split left side by '+' to get addends
    let addends: Vec<&str> = left_side.split('+').map(|s| s.trim()).collect();

    // Collect all unique letters
    let mut all_letters = std::collections::HashSet::new();
    for addend in &addends {
        for ch in addend.chars() {
            if ch.is_alphabetic() {
                all_letters.insert(ch);
            }
        }
    }
    for ch in right_side.chars() {
        if ch.is_alphabetic() {
            all_letters.insert(ch);
        }
    }

    let mut letters: Vec<char> = all_letters.into_iter().collect();
    letters.sort();

    // Need at most 10 letters (digits 0-9)
    if letters.len() > 10 {
        return None;
    }

    // Get leading letters (cannot be 0)
    let mut leading_letters = std::collections::HashSet::new();
    for addend in &addends {
        if let Some(ch) = addend.chars().find(|c| c.is_alphabetic()) {
            leading_letters.insert(ch);
        }
    }
    if let Some(ch) = right_side.chars().find(|c| c.is_alphabetic()) {
        leading_letters.insert(ch);
    }

    // Backtracking to find valid assignment
    let mut mapping = HashMap::new();
    let mut used_digits = [false; 10];
    backtrack(
        0,
        &letters,
        &leading_letters,
        &addends,
        right_side,
        &mut mapping,
        &mut used_digits,
    )
}

fn backtrack(
    idx: usize,
    letters: &[char],
    leading_letters: &std::collections::HashSet<char>,
    addends: &[&str],
    result: &str,
    mapping: &mut HashMap<char, u8>,
    used_digits: &mut [bool; 10],
) -> Option<HashMap<char, u8>> {
    if idx == letters.len() {
        // All letters assigned, check if equation is valid
        let mut total: u64 = 0;
        for addend in addends {
            let mut num: u64 = 0;
            for ch in addend.chars() {
                if ch.is_alphabetic() {
                    num = num * 10 + mapping[&ch] as u64;
                }
            }
            total += num;
        }

        let mut result_num: u64 = 0;
        for ch in result.chars() {
            if ch.is_alphabetic() {
                result_num = result_num * 10 + mapping[&ch] as u64;
            }
        }

        if total == result_num {
            return Some(mapping.clone());
        }
        return None;
    }

    let letter = letters[idx];
    let is_leading = leading_letters.contains(&letter);

    // Try each digit for this letter
    let start = if is_leading { 1 } else { 0 };
    for digit in start..10 {
        if !used_digits[digit as usize] {
            // Assign digit to letter
            used_digits[digit as usize] = true;
            mapping.insert(letter, digit);

            if let Some(result) = backtrack(idx + 1, letters, leading_letters, addends, result, mapping, used_digits) {
                return Some(result);
            }

            // Backtrack
            used_digits[digit as usize] = false;
            mapping.remove(&letter);
        }
    }

    None
}
