pub fn encode(source: &str) -> String {
    // appending a char to avoid having to deal with the last char
    format!("{source}#")
        .chars()
        .scan(
            (None, 0),
            |(last_char, count): &mut (std::option::Option<std::primitive::char>, i32), char| {
                let result = match *last_char {
                    Some(c) if c != char => {
                        // new character
                        let previous_run = format!(
                            "{}{}",
                            if *count > 1 {
                                count.to_string()
                            } else {
                                String::new()
                            },
                            c
                        );

                        *count = 1;
                        previous_run
                    }
                    //matching character or first one
                    _ => {
                        *count += 1;
                        // we don't emit anything yet
                        String::new()
                    }
                };

                *last_char = Some(char);
                Some(result)
            },
        )
        .collect::<String>()
}

pub fn decode(source: &str) -> String {
    source
        .split_inclusive(|c: char| !c.is_ascii_digit())
        .map(|s| s.split_at(s.len() - 1))
        .map(|(n, c)| c.repeat(n.parse().unwrap_or(1)))
        .collect()
}
