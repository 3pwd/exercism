use itertools::Itertools;
use std::collections::{HashMap, HashSet};

struct Factors {
    // leading digit can't be 0, so we save firts letters to check later
    pub first_letters: HashSet<char>,
    chars: Vec<char>,
    values: Vec<i64>,
}

impl Factors {
    fn new(input: &str) -> Self {
        let mut map = HashMap::new();
        let mut sign = -1;
        let mut pos = 0;

        let first_letters = input
            .split(&['+', '='])
            .filter_map(|s| s.trim().chars().next())
            .collect::<HashSet<_>>();

        for c in input.chars().filter(|c| !c.is_whitespace()).rev() {
            match c {
                '=' => {
                    sign = 1;
                    pos = 0;
                } // we are done with the left side of the equation
                '+' => {
                    pos = 0;
                } // we are done with the current term
                _ => {
                    // we get the current value for c
                    // default to 0 if not yet there
                    // update the value based on sign and pos
                    *map.entry(c).or_insert(0) += sign * 10_i64.pow(pos);
                    pos += 1;
                }
            }
        }

        let (chars, values) = map.into_iter().sorted_by_key(|(_, v)| -v.abs()).unzip();

        Self {
            chars,
            first_letters,
            values,
        }
    }

    fn firsts_letters_has_char_at(&self, i: usize) -> bool {
        self.first_letters.contains(self.chars.get(i).unwrap())
    }
}

struct Alphametics<'a> {
    input: &'a str,
}

impl<'a> Alphametics<'a> {
    fn new(input: &'a str) -> Self {
        Self { input }
    }

    fn solve(&self) -> Option<HashMap<char, u8>> {
        let factors = Factors::new(&self.input);

        for perm in (0..=9).permutations(factors.chars.len()) {
            let sum = perm
                .iter()
                .enumerate()
                .map(|(i, v)| v * factors.values.get(i).unwrap())
                .sum::<i64>();

            if sum == 0
                && !perm
                    .iter()
                    .enumerate()
                    .any(|(i, v)| *v == 0 && factors.firsts_letters_has_char_at(i))
            {
                return Some(HashMap::from_iter(
                    perm.iter()
                        .enumerate()
                        .map(|(i, v)| (*factors.chars.get(i).unwrap(), *v as u8)),
                ));
            }
        }
        None
    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    Alphametics::new(input).solve()
}
