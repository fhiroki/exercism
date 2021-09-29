pub fn abbreviate(phrase: &str) -> String {
    let word = phrase.replace("-", " ").replace("_", " ");
    let mut acronym = String::new();

    for s in word.split_whitespace() {
        acronym += &s.chars().nth(0).unwrap().to_string().to_uppercase();
        if s != s.to_uppercase() {
            for (i, c) in s.chars().enumerate() {
                if c.is_uppercase() && i != 0 {
                    acronym += &c.to_string();
                }
            }
        }
    }
    acronym
}
