use std::fmt::{Display, Formatter, Result};

pub struct Roman(String);

impl Display for Roman {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        _f.write_str(&self.0)
    }
}

impl From<u32> for Roman {
    fn from(mut num: u32) -> Self {
        let mut romans = String::from("");
        let mut p = 0;
        while num > 0 {
            romans = to_roman(num as usize % 10, p) + &romans;
            num /= 10;
            p += 1;
        }
        Roman(romans)
    }
}

fn to_roman(n: usize, p: usize) -> String {
    let tens = vec!["I", "X", "C", "M"];
    let fives = vec!["V", "L", "D"];

    match n {
        1..=3 => tens[p].repeat(n),
        4 => tens[p].to_string() + fives[p],
        5 => fives[p].to_string(),
        6..=8 => fives[p].to_string() + &tens[p].repeat(n - 5),
        9 => tens[p].to_string() + tens[p + 1],
        _ => "".to_string(),
    }
}
