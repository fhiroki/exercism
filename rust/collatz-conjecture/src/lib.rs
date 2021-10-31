pub fn collatz(mut n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut cnt = 0;
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = n.checked_mul(3)?;
            n = n.checked_add(1)?;
        }
        cnt += 1;
    }

    Some(cnt)
}
