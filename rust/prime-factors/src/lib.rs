pub fn factors(n: u64) -> Vec<u64> {
    let mut facts = vec![];
    let mut num = n;
    while num > 1 {
        for i in 2..num + 1 {
            if num % i == 0 {
                num /= i;
                facts.push(i);
                break;
            }
        }
    }
    facts
}
