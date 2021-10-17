fn pig_latin(input: &str) -> String {
    let c1 = &input[..1];
    let c2 = &input[..2];
    let c3 = &input[1..3];
    let is_vowel = "aiueo".contains(c1);

    let mut s = (&input[1..]).to_string() + c1;
    if is_vowel || "xr" == c2 || "yt" == c2 {
        s = input.to_string();
    }
    if "ch" == c2 || "qu" == c2 || "rh" == c2 {
        s = (&input[2..]).to_string() + c2;
    }
    if !is_vowel && ("qu" == c3 || "ch" == c3) {
        s = (&input[3..]).to_string() + &input[..3];
    }
    if "th" == c2 {
        if "aiueo".contains(&input[2..3]) {
            s = (&input[2..]).to_string() + c2;
        } else {
            s = (&input[3..]).to_string() + &input[..3];
        }
    }

    s + "ay"
}

pub fn translate(input: &str) -> String {
    input
        .split_whitespace()
        .map(|s| pig_latin(s))
        .collect::<Vec<String>>()
        .join(" ")
}
