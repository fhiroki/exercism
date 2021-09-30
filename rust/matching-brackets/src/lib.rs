pub fn brackets_are_balanced(string: &str) -> bool {
    let mut v = vec![];
    for c in string.chars() {
        if ['[', '{', '('].contains(&c) {
            v.push(c);
        }
        if [']', '}', ')'].contains(&c) {
            let s = v.pop().unwrap_or_default();
            let is_match = match c {
                ']' => s == '[',
                '}' => s == '{',
                ')' => s == '(',
                _ => false
            };
            if !is_match {
                return false;
            }
        }
    }
    v.is_empty()
}
