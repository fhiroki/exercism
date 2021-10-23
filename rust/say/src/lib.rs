fn encode_basic(n: u64) -> String {
    match n {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        20 => "twenty",
        30 => "thirty",
        40 => "forty",
        50 => "fifty",
        60 => "sixty",
        70 => "seventy",
        80 => "eighty",
        90 => "ninety",
        _ => "",
    }
    .to_string()
}

fn encode_under100(n: u64) -> String {
    if n < 20 || n % 10 == 0 {
        return encode_basic(n);
    }
    format!("{}-{}", encode_basic(n / 10 * 10), encode_basic(n % 10))
}

fn encode_under1000(n: u64) -> String {
    if n < 100 {
        return encode_under100(n);
    } else {
        let v = format!("{} hundred", encode_basic(n / 100));
        if n % 100 == 0 {
            return v;
        }
        return format!("{} {}", v, encode_under100(n % 100));
    }
}

pub fn encode(mut n: u64) -> String {
    if n == 0 {
        return String::from("zero");
    }

    let mut result = String::from("");
    let mut i = 0;
    while n > 0 {
        let scale = match i {
            1 => "thousand",
            2 => "million",
            3 => "billion",
            4 => "trillion",
            5 => "quadrillion",
            6 => "quintillion",
            _ => "",
        };

        let v = encode_under1000(n % 1000);
        if v != "" {
            if i == 0 {
                result = v;
            } else {
                result = format!("{} {} {}", v, scale, result);
            }
        }

        n /= 1000;
        i += 1;
    }

    result.trim().to_string()
}
