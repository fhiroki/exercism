/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let digits = isbn.replace("-", "");
    if digits.len() != 10 {
        return false;
    }

    let mut sum = 0;
    for (i, c) in digits.chars().enumerate() {
        if i == 9 && c == 'X' {
            sum += 10;
            break;
        }
        match c.to_digit(10) {
            Some(x) => sum += x * (10 - i) as u32,
            None => return false
        }
    }

    sum % 11 == 0
}
