/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .replace(" ", "")
        .replace(",", "")
        .replace(".", "")
        .to_lowercase()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let mut encoded = c;
            if c.is_alphabetic() {
                encoded = ('z' as i32 - c as i32 + 'a' as i32) as u8 as char;
            }
            format!("{}{}", if i != 0 && i % 5 == 0 { " " } else { "" }, encoded)
        })
        .collect()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .replace(" ", "")
        .chars()
        .map(|c| {
            if c.is_alphabetic() {
                ('z' as i32 - c as i32 + 'a' as i32) as u8 as char
            } else {
                c
            }
        })
        .collect()
}
