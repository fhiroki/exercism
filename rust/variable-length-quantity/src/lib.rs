#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut res = vec![];

    for value in values {
        let mut bytes = vec![];
        let vec = format!("{:b}", value)
            .chars()
            .rev()
            .collect::<Vec<char>>()
            .chunks(7)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>();

        for (i, v) in vec.iter().enumerate() {
            let s = v.chars().rev().collect::<String>();
            let b = u8::from_str_radix(&format!("{}{:0>7}", (i != 0) as u8, s), 2).unwrap();
            bytes.insert(0, b);
        }

        res.extend(bytes);
    }

   res
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut res = vec![];
    let mut s = String::from("");

    for byte in bytes {
        let b = format!("{:0>8b}", byte);
        match b.chars().nth(0) {
            Some('0') => {
                s += &b[1..];
                match u32::from_str_radix(&s, 2) {
                    Ok(n) => res.push(n),
                    Err(_) => return Err(Error::Overflow),
                }
                s = "".to_string();
            }
            _ => {
                s += &b[1..];
            }
        }
    }

    if !s.is_empty() {
        return Err(Error::IncompleteNumber);
    }

    Ok(res)
}
