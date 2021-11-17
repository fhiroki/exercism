pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_alphabetic() {
                let a = if c.is_uppercase() { 'A' } else { 'a' } as i8;
                let mut n = c as i8 - a + key;
                if n >= 26 {
                    n -= 26;
                } else if n < 0 {
                    n += 26;
                }
                (a + n) as u8 as char
            } else {
                c
            }
        })
        .collect()
}
