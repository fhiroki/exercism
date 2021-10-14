#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factors: Vec<(u64, u64)>,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        Palindrome {
            value: a * b,
            factors: vec![(a, b)],
        }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        self.factors.push((a, b));
    }
}

fn is_palindrome(n: u64) -> bool {
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut pmin = Palindrome::new(min, max);
    let mut pmax = Palindrome::new(0, 0);

    for i in min..=max {
        for j in i..=max {
            let n = i * j;
            if pmin.value() == n {
                pmin.insert(i, j);
            } else if pmin.value() > n && is_palindrome(n) {
                pmin = Palindrome::new(i, j);
            }
            if pmax.value() == n {
                pmax.insert(i, j);
            } else if pmax.value() < n && is_palindrome(n) {
                pmax = Palindrome::new(i, j);
            }
        }
    }

    if pmax.value() == 0 {
        None
    } else {
        Some((pmin, pmax))
    }
}
