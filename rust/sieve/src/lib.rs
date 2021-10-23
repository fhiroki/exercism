pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut marked = vec![false; upper_bound as usize + 1];
    (2..=upper_bound).fold(vec![], |mut result, i| {
        if !marked[i as usize] {
            result.push(i);
        }
        let mut j = 2;
        while i * j <= upper_bound {
            marked[(i * j) as usize] = true;
            j += 1;
        }
        result
    })
}
