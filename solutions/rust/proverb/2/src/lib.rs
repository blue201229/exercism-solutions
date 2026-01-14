

pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();
    if list.len() == 0{
        return proverb;
    }
    
    let _str_count = list.len();
    for i in 0.._str_count - 1 {
        proverb.push_str(&format!("For want of a {} the {} was lost.\n", list[i], list[i+1]));
    }
    proverb.push_str(&format!("And all for the want of a {}.", list[0]));
    proverb
}
