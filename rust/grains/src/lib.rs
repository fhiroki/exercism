pub fn square(s: u32) -> u64 {
    if s <= 0 || s > 64 {
        panic!("Square must be between 1 and 64")
    }
    2u64.pow(s - 1)
}

pub fn total() -> u64 {
    let mut sum : u64 = 0;
    let mut s : u64 = 1;
    for i in 1..65 {
        sum += s;
        if i != 64 {
            s *= 2;
        }
    }
    sum
}
