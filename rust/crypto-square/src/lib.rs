pub fn encrypt(input: &str) -> String {
    let normalized: String = input
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic() || c.is_numeric())
        .collect();

    let len = normalized.len();
    let col = (len as f32).sqrt().ceil() as usize;
    let row = if col * col == len { col } else { col - 1 };

    let mut coded = String::from("");
    let mut i = 0;
    for x in 0..col {
        for y in (0..len).step_by(col) {
            if i != 0 && i % row == 0 {
                coded += " ";
            }
            coded += &normalized.chars().nth(x + y).unwrap_or(' ').to_string();
            i += 1;
        }
    }

    coded
}
