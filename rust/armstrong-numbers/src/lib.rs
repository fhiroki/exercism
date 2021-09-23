pub fn is_armstrong_number(num: u32) -> bool {
    let size = num.to_string().len() as u32;
    let mut n = num;
    let mut sum = 0;
    while n > 0 {
        sum += (n % 10).pow(size);
        n /= 10;
    }
    sum == num
}
