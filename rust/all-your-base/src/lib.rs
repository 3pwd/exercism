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

    if number.is_empty() {
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

    if decimal == 0 {
        return Ok(vec![0]);
    }

    let mut result: Vec<u32> = Vec::new();

    while decimal != 0 {
        result.push(decimal % to_base);
        decimal /= to_base; // behaves like floor division for positive integers
    }
    result.reverse();
    Ok(result)
}
