fn is_prime(n: u32) -> bool {
    if n == 2 {
        return true;
    }
    let end = (n as f64).sqrt().ceil() as u32 + 1;
    for i in 2..end {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn nth(n: u32) -> u32 {
    let mut num = 0;
    for i in 2..1000000 {
        if is_prime(i) {
            num += 1;
        }
        if num - 1 == n {
            return i;
        }
    }
    0
}
