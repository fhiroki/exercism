fn check_valid(s: &str) -> bool {
    for i in vec![0, 3] {
        let c = s.chars().nth(i).unwrap();
        if c == '0' || c == '1' {
            return false;
        }
    }
    true
}

pub fn number(user_number: &str) -> Option<String> {
    let mut res = user_number
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>();

    if res.len() == 11 && res.starts_with('1') {
        res.remove(0);
    }

    if res.len() == 10 && check_valid(&res) {
        Some(res)
    } else {
        None
    }
}
