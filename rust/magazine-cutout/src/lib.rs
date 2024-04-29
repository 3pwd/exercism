use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_map: HashMap<&str, i8> = HashMap::new();

    for word in magazine.iter() {
        magazine_map
            .entry(word)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    for word in note.iter() {
        match magazine_map.entry(word).or_default() {
            0 => {
                return false;
            }
            count => *count -= 1,
        }
    }

    true
}
