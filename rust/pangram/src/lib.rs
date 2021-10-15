/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    for c in 'a'..='z' {
        if !sentence.to_lowercase().contains(c) {
            return false;
        }
    }
    true
}
