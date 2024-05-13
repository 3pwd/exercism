pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut num = n;
    for step in 0.. {
        num = match num {
            1 => return Some(step),
            val if val % 2 == 0 => num / 2,
            _ => num.checked_mul(3)?.checked_add(1)?,
        }
    }
    None
}
