/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    match find_mmi(a, 26) {
        Some(_) => (),
        None => return Err(AffineCipherError::NotCoprime(a)),
    };

    Ok(plaintext
        .replace(" ", "")
        .replace(",", "")
        .replace(".", "")
        .to_lowercase()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let mut encoded = c;
            if c.is_alphabetic() {
                let x = c as i32 - 'a' as i32;
                let e = ((a * x + b) % 26) as u8;
                encoded = (e + 'a' as u8) as char;
            }
            format!("{}{}", if i != 0 && i % 5 == 0 { " " } else { "" }, encoded)
        })
        .collect())
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let n = match find_mmi(a, 26) {
        Some(x) => x,
        None => return Err(AffineCipherError::NotCoprime(a)),
    };

    Ok(ciphertext
        .replace(" ", "")
        .chars()
        .map(|c| {
            if c.is_alphabetic() {
                let y = c as i32 - 'a' as i32;
                let mut e = (n * (y - b) % 26) as i32;
                if e < 0 {
                    e += 26;
                }
                (e as u8 + 'a' as u8) as char
            } else {
                c
            }
        })
        .collect())
}

fn find_mmi(a: i32, m: i32) -> Option<i32> {
    for n in 1..m {
        if a * n % m == 1 {
            return Some(n);
        }
    }
    None
}
