pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let students = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];
    let apply_char = |c:char| match c {
        'V' => "violets",
        'R' => "radishes",
        'C' => "clover",
        'G' => "grass",
        _   => ""
    };
    let student_index = 2 * students.iter().position(|&s| s == student).unwrap();
    diagram.lines().flat_map(|line|
        {
            println!("{}",line);
        line[student_index..=student_index+1].chars().map(apply_char)
    })
    .collect()
}
