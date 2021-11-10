pub struct Luhn {
    nums: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        if self.nums.len() <= 1 {
            return false;
        }

        let mut sum = 0;
        for (i, c) in self.nums.chars().enumerate() {
            let mut n = match c.to_digit(10) {
                Some(n) => n,
                None => return false,
            };
            if i % 2 == self.nums.len() % 2 {
                n *= 2;
                if n > 9 {
                    n -= 9;
                }
            }
            sum += n;
        }
        sum % 10 == 0
    }
}

impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Luhn {
            nums: input.to_string().replace(" ", ""),
        }
    }
}
