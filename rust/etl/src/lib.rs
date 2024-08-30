use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter().fold(BTreeMap::new(), |mut t, (score, letters)| {
        letters.iter().for_each(|c| {
            t.insert(c.to_ascii_lowercase(), *score);
        });
        t
    })
}
