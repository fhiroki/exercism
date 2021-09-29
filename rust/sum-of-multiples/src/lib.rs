use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sums = HashSet::new();
    for n in factors {
        if *n == 0 {
            continue;
        }
        let mut i = 1;
        while n * i < limit {
            sums.insert(n * i);
            i += 1;
        }
    }
    let mut sum: u32 = 0;
    for n in sums {
        sum += n;
    }
    sum
}
