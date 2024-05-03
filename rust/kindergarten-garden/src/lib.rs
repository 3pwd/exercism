use std::collections::HashMap;

const STUDENTS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];
const PLANTS: [&str; 4] = ["violets", "radishes", "grass", "clover"];

pub struct Garden<'a> {
    layout: [[char; 4]; 12],
    plants: HashMap<char, &'a str>,
    students: HashMap<&'a str, usize>,
}

impl Garden<'static> {
    pub fn new(diagram: String) -> Self {
        let mut plants = HashMap::new();
        let mut students = HashMap::new();

        let mut layout: [[char; 4]; 12] = [[' '; 4]; 12];
        for (i, line) in diagram.lines().enumerate() {
            for (j, chunk) in line.chars().collect::<Vec<char>>().chunks(2).enumerate() {
                layout[j][i * 2] = chunk[0];
                layout[j][i * 2 + 1] = chunk[1];
            }
        }
        for plant in &PLANTS {
            plants.insert(plant.chars().next().unwrap(), *plant);
        }

        for (i, student) in STUDENTS.iter().enumerate() {
            students.insert(*student, i);
        }

        Self {
            layout,
            plants,
            students,
        }
    }

    fn plants(&self, student: &str) -> Vec<&'static str> {
        let index = self
            .students
            .get(student)
            .expect("this student is not in the class");
        self.layout[*index]
            .iter()
            .map(|&plant| self.plants[&plant])
            .collect()
    }
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    Garden::new(diagram.to_lowercase()).plants(student)
}
