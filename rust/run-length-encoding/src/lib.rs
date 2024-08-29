pub fn encode(source: &str) -> String {
    let mut result = source
        .chars()
        .scan((None, 0), |(last_char, count): &mut(std::option::Option<std::primitive::char>, i32), char| {
            let result = match *last_char {
                //matching character
                Some(c) if c == char => {
                    *count += 1;
                    // we don't emit anything yet
                    String::new()
                }
                Some(c) => {
                    // new character
                    let previous_run = if *count > 1 {
                        format!("{count}{c}")
                    } else {
                        c.to_string()
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
     source.chars().scan(String::new(), |acc, c| {
        if c.is_ascii_digit() {
            acc.push(c);
            Some(String::new())
        } else {
            let count = acc.parse().unwrap_or(1);
            acc.clear();
            Some(c.to_string().repeat(count))
        }
    }).collect()

}
