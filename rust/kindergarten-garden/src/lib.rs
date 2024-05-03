const STUDENTS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

pub fn get_plant(c: char) -> &'static str {
    match c {
        'V' => "violets",
        'R' => "radishes",
        'G' => "grass",
        _ => "clover",
    }
}

pub fn get_plants(s: &str) -> Vec<&'static str> {
    s.chars().map(get_plant).collect()
}

pub fn find_cup_idx(student: &str) -> usize {
    STUDENTS.iter().position(|&x| x == student).unwrap() * 2
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    diagram
        .lines()
        .flat_map(|l| l.chars().skip(find_cup_idx(student)).take(2).map(get_plant))
        .collect()
}
