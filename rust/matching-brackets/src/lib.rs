use std::collections::HashMap;

const BRACKET_PAIRS: [(char, char); 4] = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')];

struct Brackets {
    brackets: Vec<char>,
    pairs: HashMap<char, char>,
    stack: Vec<char>,
}

impl Brackets {
    fn new(string: &str) -> Self {
        let brackets = string
            .chars()
            .filter(|&ch| BRACKET_PAIRS.iter().any(|&(l, r)| l == ch || r == ch))
            .collect::<Vec<char>>();
        Self {
            brackets,
            pairs: HashMap::from(BRACKET_PAIRS),
            stack: Vec::new(),
        }
    }

    fn are_balanced(&mut self) -> bool {
        for &ch in &self.brackets {
            if let Some(&expected_closing_bracket) = self.pairs.get(&ch) {
                self.stack.push(expected_closing_bracket);
            } else if self.stack.pop() != Some(ch) {
                return false;
            }
        }
        self.stack.is_empty()
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    Brackets::new(string).are_balanced()
}
