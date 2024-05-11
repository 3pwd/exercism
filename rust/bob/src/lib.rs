pub fn is_yelling(message: &str) -> bool {
    message.chars().any(|c| c.is_alphabetic()) && message.to_uppercase() == message
}

pub fn reply(message: &str) -> &str {
    match message.trim() {
        m if m.is_empty() => "Fine. Be that way!",
        m if is_yelling(m) && m.ends_with("?") => "Calm down, I know what I'm doing!",
        m if is_yelling(m) => "Whoa, chill out!",
        m if m.ends_with("?") => "Sure.",
        _ => "Whatever.",
    }
}
