pub fn raindrops(n: u32) -> String {
    let sounds = [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .iter()
        .filter_map(|&(divisor, sound)| match n % divisor {
            0 => Some(sound),
            _ => None,
        })
        .collect::<Vec<&str>>()
        .join("");

    match sounds.is_empty() {
        true => n.to_string(),
        _ => sounds,
    }
}
