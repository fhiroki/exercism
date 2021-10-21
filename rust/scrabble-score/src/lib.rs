/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    word.to_lowercase().chars().fold(0, |mut point, c| {
        if c.is_ascii_alphabetic() {
            point += match c {
                'd' | 'g' => 2,
                'b' | 'c' | 'm' | 'p' => 3,
                'f' | 'h' | 'v' | 'w' | 'y' => 4,
                'k' => 5,
                'j' | 'x' => 8,
                'q' | 'z' => 10,
                _ => 1,
            };
        }
        point
    })
}
