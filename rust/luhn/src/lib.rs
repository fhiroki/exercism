/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let s = code.replace(" ", "");
    if s.len() <= 1 {
        return false;
    }

    let mut sum = 0;
    for (i, c) in s.chars().enumerate() {
        let mut n = match c.to_digit(10) {
            Some(n) => n,
            None => return false,
        };
        if i % 2 == s.len() % 2 {
            n *= 2;
            if n > 9 {
                n -= 9;
            }
        }
        sum += n;
    }

    sum % 10 == 0
}
