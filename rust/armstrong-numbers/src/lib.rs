pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    let len = num_str.len() as u32;
    match num_str
        .chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .try_fold(0_u32,  |acc, x|acc.checked_add(x.pow(len))) {
        Some(result) => result == num,
        _ => false
    }
}
