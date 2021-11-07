pub fn get_diamond(c: char) -> Vec<String> {
    let mut diamond = vec![];
    let n = (c as usize - 'A' as usize + 1) * 2 - 1;
    for (i, l) in ('A'..=c).rev().enumerate() {
        let s;

        if l == 'A' {
            let space = " ".repeat((n - 1) / 2);
            s = format!("{}{}{}", space, l, space);
        } else {
            let m = (l as usize - 'A' as usize) * 2 - 1;
            let space = " ".repeat((n - m - 2) / 2);
            s = format!("{}{}{}{}{}", space, l, " ".repeat(m), l, space);
        }

        if i != 0 {
            diamond.insert(0, s.to_string());
            diamond.push(s.to_string());
        } else {
            diamond.push(s.to_string());
        }
    }
    diamond
}
