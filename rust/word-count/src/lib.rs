use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    for word in words.split(&[' ', ',', ':', '\n'][..]) {
        let s = word
            .chars()
            .enumerate()
            .filter(|(i, c)| c.is_alphanumeric() || c == &'\'' && i != &0 && i != &(word.len() - 1))
            .map(|(_, c)| c)
            .collect::<String>()
            .to_lowercase();

        if s.is_empty() {
            continue;
        }
        let counter = map.entry(s.to_string()).or_insert(0);
        *counter += 1;
    }
    map
}
