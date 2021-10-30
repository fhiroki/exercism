// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let lines: Vec<&str> = input.split('\n').collect();
    let row = lines.len();
    if row % 4 != 0 {
        return Err(Error::InvalidRowCount(row));
    }
    let col = lines[0].len();
    if col % 3 != 0 {
        return Err(Error::InvalidColumnCount(col));
    }

    let mut map = HashMap::new();
    #[rustfmt::skip]
    let answer = " _     _  _     _  _  _  _  _ \n".to_string() +
                 "| |  | _| _||_||_ |_   ||_||_|\n" +
                 "|_|  ||_  _|  | _||_|  ||_| _|\n";
    for (i, j) in (0..30).step_by(3).enumerate() {
        let s = answer[j..j + 3].to_string()
            + &answer[j + 31..j + 31 + 3]
            + &answer[j + 62..j + 62 + 3];
        map.insert(s, i.to_string());
    }

    let mut res = String::from("");
    for h in (0..row).step_by(4) {
        for w in (0..col).step_by(3) {
            let mut s = String::from("");
            for i in 0..3 {
                s += &lines[h + i][w..w + 3];
            }
            match map.get(&s) {
                Some(n) => res += n,
                _ => res += "?",
            }
        }
        if h != row - 4 {
            res += ",";
        }
    }

    return Ok(res);
}
