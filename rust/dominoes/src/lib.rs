use std::vec;

use itertools::Itertools;

pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    let n = input.len();
    if n == 0 {
        return Some(vec![]);
    } else if n == 1 {
        if input[0].0 == input[0].1 {
            return Some(input.to_vec());
        } else {
            return None;
        }
    }

    for perm in input.iter().permutations(n) {
        let mut ok = true;
        let mut now = *perm[0];
        let mut next = *perm[1];
        let mut ans = vec![now];
        for i in 1..=n {
            if now.1 == next.0 || now.1 == next.1 {
                if i == n {
                    continue;
                }
                if now.1 == next.0 {
                    now = next;
                } else {
                    now = (next.1, next.0);
                }
                next = if i != n - 1 { *perm[i + 1] } else { *perm[0] };
                ans.push(now);
            } else {
                ok = false;
                break;
            }
        }
        if ok {
            return Some(ans);
        }
    }
    None
}
