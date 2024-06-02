pub fn abbreviate(phrase: &str) -> String {
    let mut acronym = String::new();
    let mut previous_char = ' ';
    let mut new_word = true;

    for c in phrase.chars() {
        if c.is_alphabetic() {
            if new_word || (previous_char.is_lowercase() && c.is_uppercase()) {
                acronym.push(c.to_ascii_uppercase());
                new_word = false;
            }
        } else if c.is_ascii_whitespace() || c == '-' {
            new_word = true;
        }
        previous_char = c;
    }
    acronym
}
