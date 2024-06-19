use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn split<'a>(input: &'a str, delimiter: &str) -> Option<(&'a str, &'a str)> {
    let mut split = input.split(delimiter);
    let left = split.next()?;
    let right = split.next()?;
    Some((left, right))
}

pub fn find_unique_letters(input: &str) -> Vec<char> {
    let mut seen = HashSet::new();
    input
        .chars()
        .filter(|c| c.is_alphabetic() && seen.insert(*c))
        .collect()
}

pub fn convert(input: &str, map: &HashMap<char, u8>) -> usize {
    let result = input
        .chars()
        .map(|c| map.get(&c).unwrap().to_string())
        .collect::<String>();
    result.parse().unwrap()
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let cleaned = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();

    if let Some((left_side, result)) = split(&cleaned, "==") {
        let addends: Vec<&str> = left_side.split("+").collect();
        let letters = find_unique_letters(&cleaned);
        // generate permutations and filter to avoid leading zeroes early
        let perms = (0..10).permutations(letters.len()).filter(|perm| {
            for &c in &letters {
                let idx = letters.iter().position(|&x| x == c).unwrap();
                // ensure that the leading character of each addend does not map to 0
                if addends.iter().any(|&addend| addend.starts_with(c)) || result.starts_with(c) {
                    if perm[idx] == 0 {
                        return false;
                    }
                }
            }
            true
        });

        for perm in perms {
            let map: HashMap<char, u8> = letters.iter().cloned().zip(perm.clone()).collect();

            if addends
                .iter()
                .map(|addend| convert(addend, &map))
                .sum::<usize>()
                == convert(result, &map)
            {
                return Some(map);
            }
        }
    }
    None
}
