pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let lines: Vec<&str> = diagram.lines().collect();
    let students = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];
    let plants_map = [
        ('C', "clover"),
        ('G', "grass"),
        ('R', "radishes"),
        ('V', "violets"),
    ];
    let student_index = students.iter().position(|&s| s == student).unwrap();
    let mut result = Vec::new();    
    for line in &lines {
        let chars: Vec<char> = line.chars().collect();
        let fp_c = chars[student_index * 2];
        let sp_c = chars[student_index * 2 + 1];
        let mut fp_v = "";
        let mut sp_v = "";
        for &(ch, plant) in &plants_map {
            
            if ch == fp_c {
                fp_v = plant;
                
            }
            if ch == sp_c {
                sp_v = plant;
            }
        }
        result.push(fp_v);
        result.push(sp_v);
    }
    result
}
