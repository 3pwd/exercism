pub fn luhn(index: u32, digit: u32) -> u32 {
    if index % 2 == 0 {
        digit
    } else {
        let double = digit * 2;
        // double - (0 or 1) * 9
        double - (double > 9) as u32 * 9
    }
}

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .filter(|c| !c.is_whitespace())
        .try_rfold((0u32, 0u32), |(len, sum), d| {
            d.to_digit(10).map(|d| (len + 1, sum + luhn(len, d)))
        })
        .map_or(false, |(len, sum)| len > 1 && sum % 10 == 0)
}
