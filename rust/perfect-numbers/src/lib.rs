#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    let mut sum = 0;
    for i in 1..num {
        if num % i == 0 {
            sum += i;
        }
    }
    match sum {
        s if s == num => Some(Classification::Perfect),
        s if s > num => Some(Classification::Abundant),
        _ => Some(Classification::Deficient)
    }
}
