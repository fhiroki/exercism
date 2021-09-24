pub fn square_of_sum(n: u32) -> u32 {
    let sum = n * (n + 1) / 2;
    sum * sum
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut sum = 0;
    for i in 1..n + 1 {
        sum += i * i;
    }
    sum
}

pub fn difference(n: u32) -> u32 {
    let a = sum_of_squares(n);
    let b = square_of_sum(n);
    if a > b {
        return a - b;
    }
    b - a
}
