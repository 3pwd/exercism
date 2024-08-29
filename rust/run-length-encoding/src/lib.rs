pub fn encode(source: &str) -> String {
    // appending a char to avoid having to deal with the last char
    format!("{source}#")
        .chars()
        .fold(
            (String::new(), None, 0),
            |(s, last, count): (String, Option<char>, u64), c| match last {
                Some(last) if last == c => (s, Some(last), count + 1),
                Some(last) => (
                    format!(
                        "{}{}{}",
                        s,
                        if count > 1 {
                            count.to_string()
                        } else {
                            String::new()
                        },
                        last
                    ),
                    Some(c),
                    1,
                ),
                None => (s, Some(c), 1),
            },
        )
        .0
}

pub fn decode(source: &str) -> String {
    source
        .split_inclusive(|c: char| !c.is_ascii_digit())
        .map(|s| s.split_at(s.len() - 1))
        .map(|(n, c)| c.repeat(n.parse().unwrap_or(1)))
        .collect()
}
