#[derive(Debug)]
struct Minefield {
    pub annotations: Vec<String>,
    pub field: Vec<String>,
}

impl Minefield {
    pub fn new(field: &[&str]) -> Self {
        Self {
            annotations: field
                .iter()
                .map(|&s| String::with_capacity(s.len()))
                .collect::<Vec<String>>(),
            field: field.iter().map(|&s| s.to_owned()).collect(),
        }
    }

    fn neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for i in x.saturating_sub(1)..=(x + 1).min(self.field.len() - 1) {
            for j in y.saturating_sub(1)..=(y + 1).min(self.field[i].len() - 1) {
                if !(i == x && j == y) && self.field[i].as_bytes()[j] == b'*' {
                    count += 1;
                }
            }
        }
        count
    }

    fn is_mine(&self, x: usize, y: usize) -> bool {
        self.field[x].as_bytes()[y] == b'*'
    }

    fn annotate_cell(&mut self, x: usize, y: usize) {
        let annotation = if self.is_mine(x, y) {
            '*'.to_string()
        } else {
            let neighbors = self.neighbors(x, y);
            if neighbors == 0 {
                " ".to_string()
            } else {
                neighbors.to_string()
            }
        };
        self.annotations[x].push_str(&annotation);
    }

    pub fn solve(&mut self) {
        for x in 0..self.field.len() {
            for y in 0..self.field[x].len() {
                self.annotate_cell(x, y);
            }
        }
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut field = Minefield::new(minefield);
    field.solve();
    field.annotations
}
