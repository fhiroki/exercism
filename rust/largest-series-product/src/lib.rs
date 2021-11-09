#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    let n = string_digits.len();
    if n < span {
        return Err(Error::SpanTooLong);
    }

    let mut v = vec![];
    for c in string_digits.chars() {
        match c.to_digit(10) {
            Some(x) => v.push(x as u64),
            None => return Err(Error::InvalidDigit(c)),
        }
    }

    let mut max_prod: u64 = 0;
    for i in 0..=n - span {
        let mut prod: u64 = 1;
        for j in i..i + span {
            prod *= v[j];
        }
        if prod > max_prod {
            max_prod = prod;
        }
    }

    Ok(max_prod)
}
