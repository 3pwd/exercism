pub fn factors(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut result: Vec<u64> = Vec::new();
    let mut current_factor = 2;

    while n > 1 {
        while n % current_factor == 0 {
            result.push(current_factor);
            n = n / current_factor;
        }
        current_factor += 1;
    }

    result
}
