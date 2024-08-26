use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn calc_factors(input: &str) -> (Vec<char>, Vec<i64>) {
    let mut factors = HashMap::new();
    let mut sign = -1;
    let mut pos = 0;

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
                *factors.entry(c).or_insert(0) += sign * 10_i64.pow(pos);
                pos += 1;
            }
        }
    }
    // sort pairs by absolute value of values in descending order (-)
    factors.into_iter().sorted_by_key(|(_, v)| -v.abs()).unzip()
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // leading digit can't be 0, so we save firts letters to check later
    let firsts = input
        .split(&['+', '='])
        .filter_map(|s| s.trim().chars().next())
        .collect::<HashSet<_>>();
    let (letters, factors) = calc_factors(&input);

    for perm in (0..=9).permutations(letters.len()) {
        let sum = perm
            .iter()
            .enumerate()
            .map(|(i, v)| v * factors.get(i).unwrap())
            .sum::<i64>();
        if sum == 0
            && !perm
                .iter()
                .enumerate()
                .any(|(i, v)| *v == 0 && firsts.contains(letters.get(i).unwrap()))
        {
            return Some(HashMap::from_iter(
                perm.iter()
                    .enumerate()
                    .map(|(i, v)| (*letters.get(i).unwrap(), *v as u8)),
            ));
        }
    }
    None
}
