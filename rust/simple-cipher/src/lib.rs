use rand::Rng;

fn translate(key: &str, s: &str, f: fn(u8, u8) -> u8) -> Option<String> {
    if key.is_empty() || !key.chars().all(|c| c.is_alphabetic() && c.is_lowercase()) {
        return None;
    }

    s.as_bytes()
        .iter()
        .zip(key.as_bytes().iter().cycle())
        .map(|(c, k)| Some((b'a' + f(26 + c - b'a', k - b'a') % 26) as char))
        .collect()
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    translate(key, s, u8::wrapping_add)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    translate(key, s, u8::wrapping_sub)
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = rand::thread_rng();
    let key = (0..100).fold(String::from(""), |mut s, _| {
        let c = rng.gen_range(b'a', b'z') as char;
        s += &c.to_string();
        s
    });
    (key.clone(), encode(&key, s).unwrap())
}
