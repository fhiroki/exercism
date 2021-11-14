use std::collections::HashMap;

#[derive(Debug)]
struct Letter {
    letter: char,
    factor: i64,
    start: u8,
}

fn guess(letters: &[Letter], letters_value: &mut Vec<u8>, equation_value: i64) -> Option<Vec<u8>> {
    if letters.is_empty() {
        return if equation_value == 0 {
            Some(letters_value.clone())
        } else {
            None
        };
    }
    let factor = letters[0].factor;
    if factor < 0 && equation_value < 0 {
        return None;
    }

    // Brute force recursive loop
    for n in letters[0].start..=9 {
        if !letters_value.contains(&n) {
            letters_value.push(n);
            let new_equation_value = equation_value + n as i64 * factor;
            let answer = guess(&letters[1..], letters_value, new_equation_value);
            if answer.is_some() {
                return answer;
            }
            letters_value.pop();
        }
    }
    None
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // Will compose a 1st degree equation to sum all letters from words
    let mut letters: HashMap<char, Letter> = HashMap::new();
    let mut right = false;
    for word in input.split_whitespace() {
        match word {
            "=" | "==" => right = true,
            "+" => (),
            w => {
                let word_length = w.len();
                let mut factor: i64 = 10i64.pow((word_length - 1) as u32);
                if right {
                    factor *= -1;
                }
                for (pos, letter) in w.chars().enumerate() {
                    if let Some(letter) = letters.get_mut(&letter) {
                        letter.factor += factor;
                    } else {
                        if letters.len() == 10 {
                            // Too many letters
                            return None;
                        }
                        letters.insert(
                            letter,
                            Letter {
                                letter,
                                factor,
                                start: if pos == 0 && word_length > 1 { 1 } else { 0 },
                            },
                        );
                    }
                    factor /= 10;
                }
            }
        }
    }
    let mut letters: Vec<Letter> = letters.into_iter().map(|(_, value)| value).collect();
    // Sorting in descending order for positive factors and ascending for megative factors.
    // This will be used to detect when the equations result is already negative and there's
    // no more positive factors, so we can early break the brute force loop.
    letters.sort_by(|left, right| {
        if right.factor < 0 && left.factor < 0 {
            left.factor.cmp(&right.factor)
        } else {
            right.factor.cmp(&left.factor)
        }
    });

    let mut answer = Vec::new();
    if let Some(letters_value) = guess(&letters, &mut answer, 0) {
        return Some(
            letters
                .iter()
                .map(|letter| letter.letter)
                .zip(letters_value.into_iter())
                .collect(),
        );
    }
    None
}
