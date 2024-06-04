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

    let mut decimal = number
        .iter()
        .try_fold(0u32, |acc, &d| match d >= from_base {
            true => Err(Error::InvalidDigit(d)),
            false => Ok(acc * from_base + d),
        })?;

    let mut result = vec![];

    loop {
        result.insert(0, decimal % to_base);
        decimal /= to_base;
        if decimal == 0 {
            break Ok(result);
        }
    }
}
