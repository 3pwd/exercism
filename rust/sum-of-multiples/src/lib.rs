use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = HashSet::new();

    for factor in factors.iter().filter(|&f| *f != 0) {
        let mut i = 1;
        while factor * i < limit {
            multiples.insert(factor * i);
            i += 1;
        }
    }

    multiples.iter().sum()
}
