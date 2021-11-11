pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        is_valid(self.to_string().replace(" ", ""))
    }
}

fn is_valid(input: String) -> bool {
    if input.len() <= 1 {
        return false;
    }
    let mut sum = 0;
    for (i, c) in input.chars().enumerate() {
        let mut n = match c.to_digit(10) {
            Some(n) => n,
            None => return false,
        };
        if i % 2 == input.len() % 2 {
            n *= 2;
            if n > 9 {
                n -= 9;
            }
        }
        sum += n;
    }
    sum % 10 == 0
}
