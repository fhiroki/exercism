pub fn encode(source: &str) -> String {
    let mut result = String::from("");
    let mut cnt = 1;
    for i in 0..source.len() {
        let now = source.chars().nth(i).unwrap();
        if i == source.len() - 1 {
            if cnt != 1 {
                result += &cnt.to_string();
            }
            result.push(now);
            break;
        }

        let next = source.chars().nth(i + 1).unwrap();
        if now == next {
            cnt += 1;
        } else {
            if cnt != 1 {
                result += &cnt.to_string();
            }
            result.push(now);
            cnt = 1;
        }
    }
    result.to_string()
}

pub fn decode(source: &str) -> String {
    let mut vec = vec![];
    source.chars().fold(String::from(""), |mut result, c| {
        if c.is_numeric() {
            vec.push(c.to_digit(10).unwrap());
        } else {
            let mut num = vec.iter().fold(0, |acc, n| acc * 10 + n);
            if num == 0 {
                num += 1;
            }
            vec.clear();
            for _ in 0..num {
                result.push(c);
            }
        }
        result
    })
}
