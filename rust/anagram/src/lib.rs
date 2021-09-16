use std::collections::HashSet;

fn sort_string(word: &str) -> String {
    let mut words: Vec<_> = word.to_lowercase().chars().collect();
    words.sort();
    words.into_iter().collect()
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();

    let sorted_word = sort_string(word);
    for s in possible_anagrams {
        let y = sort_string(s);
        if sorted_word == y && word.to_lowercase() != s.to_lowercase() {
            anagrams.insert(*s);
        }
    }
    anagrams
}
