#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if to_base <= 1 { return Err(Error::InvalidOutputBase) }
    if from_base <= 1 { return Err(Error::InvalidInputBase) }
    let mut n: u32 = 0;
    for &digit in number.iter() {
        if digit >= from_base { return Err(Error::InvalidDigit(digit)) }
        n = (n * from_base) + digit;
    }

    let mut output = vec![];
    while n > 0 {
        output.push(n % to_base);
        n /= to_base;
    }
    if output.is_empty() { output.push(0) }

    Ok(output.into_iter().rev().collect())
}
