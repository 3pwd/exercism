pub fn encode(source: &str) -> String {
    let mut result = source
        .chars()
        .scan((None, 0), |(last_char, count), char| {
            let result = match *last_char {
                //matching character
                Some(c) if c == char => {
                    *count += 1;
                    // we don't emit anything yet
                    String::new()
                }
                Some(c) => {
                    // new character
                    let previous_run = match *count > 1 {
                        false => String::from(c),
                        true => format!("{count}{c}"),
                    };
                    *count = 1;
                    previous_run
                }
                None => {
                    *count = 1;
                    String::new()
                }
            };

            *last_char = Some(char);
            Some(result)
        })
        .collect::<String>();

    // emit last run
    if let Some(last_char) = source.chars().last() {
        let last_count = source.chars().rev().take_while(|&c| c == last_char).count();
        if last_count > 1 {
            result.push_str(&format!("{last_count}{last_char}"));
        } else {
            result.push_str(&format!("{last_char}"));
        }
    }

    result
}

pub fn decode(source: &str) -> String {
    let mut result = String::new();
    let mut count_str = String::new();

    for ch in source.chars() {
        if ch.is_ascii_digit() {
            count_str.push(ch);
        } else {
            let count = count_str.parse().unwrap_or(1);
            result.push_str(&ch.to_string().repeat(count));
            count_str.clear();
        }
    }

    result
}
