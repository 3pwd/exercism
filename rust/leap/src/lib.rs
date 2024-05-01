pub fn is_divisible_by(divider: u64) -> impl Fn(u64) -> bool {
    move |number: u64| number % divider == 0
}

pub fn is_divisible_by_four() -> impl Fn(u64) -> bool {
    is_divisible_by(4)
}
pub fn is_divisible_by_one_hundred() -> impl Fn(u64) -> bool {
    is_divisible_by(100)
}
pub fn is_divisible_by_four_hundreds() -> impl Fn(u64) -> bool {
    is_divisible_by(400)
}

pub fn is_leap_year(year: u64) -> bool {
    let divisible_by_four = is_divisible_by_four();
    let divisible_by_one_hundred = is_divisible_by_one_hundred();
    let divisible_by_four_hundreds = is_divisible_by_four_hundreds();

    divisible_by_four(year) && !divisible_by_one_hundred(year) || divisible_by_four_hundreds(year)
}
