pub fn reply(message: &str) -> &str {
    let s = message.trim();

    let mut is_upper = true;
    let mut has_alphabet = false;
    for c in s.chars() {
        if c.is_alphabetic() {
            has_alphabet = true;
            if !c.is_uppercase() {
                is_upper = false;
            }
        }
    }

    if s.is_empty() {
        return "Fine. Be that way!";
    }

    let is_question = s.chars().last().unwrap() == '?';
    if is_upper && has_alphabet {
        if is_question {
            return "Calm down, I know what I'm doing!";
        } else {
            return "Whoa, chill out!";
        }
    } else if is_question {
        return "Sure.";
    }
    "Whatever."
}
