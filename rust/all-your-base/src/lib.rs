#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }

    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    if number.is_empty() || number.iter().all(|&d| d == 0) {
        return Ok(vec![0]);
    }

    let mut decimal = number
        .iter()
        .rev()
        .enumerate()
        .try_fold(0u32, |acc, (i, &d)| match d >= from_base {
            true => Err(Error::InvalidDigit(d)),
            false => Ok(acc + d * from_base.pow(i as u32)),
        })?;

    let mut result = vec![];

    while decimal > 0 {
        result.insert(0, decimal % to_base);
        decimal /= to_base;
    }

    Ok(result)
}
