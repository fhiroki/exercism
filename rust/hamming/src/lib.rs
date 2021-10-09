/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }

    let dist = s1
        .chars()
        .zip(s2.chars())
        .fold(0, |sum, (x, y)| sum + (x != y) as usize);
    Some(dist)
}
