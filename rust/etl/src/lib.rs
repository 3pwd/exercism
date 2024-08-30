use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut th: BTreeMap<char, i32> = BTreeMap::new();

    for (score, letters) in h.iter() {
        for letter in letters.iter() {
            let lowercase_letter = letter.to_ascii_lowercase();
            th.insert(lowercase_letter, *score);
        }
    }

    th
}
