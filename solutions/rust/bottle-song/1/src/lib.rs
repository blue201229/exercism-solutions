
pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut song_str = String::new();
    const WORDS: [&str; 11] = [
        "No", "One", "Two", "Three", "Four", 
        "Five", "Six", "Seven", "Eight", "Nine", "Ten"
    ];
    for i in 1..=take_down {
        let current_bottles = start_bottles - i + 1;
        song_str.push_str(&format!("{} green bottle{} hanging on the wall,\n", WORDS[current_bottles as usize], if current_bottles > 1 {"s"} else {""}));
        song_str.push_str(&format!("{} green bottle{} hanging on the wall,\n", WORDS[current_bottles as usize], if current_bottles > 1 {"s"} else {""}));
        song_str.push_str("And if one green bottle should accidentally fall,\n");

        println!("f {}", current_bottles);
        song_str.push_str(&format!("There'll be {} green bottle{} hanging on the wall.\n", WORDS[current_bottles as usize- 1].to_lowercase(), if current_bottles == 2 {""} else {"s"}));
     
        if i < take_down {
            song_str.push_str("\n");
            
        }
    }
    song_str
}