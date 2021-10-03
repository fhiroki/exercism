use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let v: Vec<&str> = input.trim().split("==").collect();
    let left = v[0].split("+");
    let right = v[1];
    let mut left_map : HashMap<char, u64> = HashMap::new();
    let mut right_map : HashMap<char, u64> = HashMap::new();
    for s in left {
        let mut i = 1;
        for c in s.trim().chars().rev() {
            if left_map.contains_key(&c) {
                *left_map.get_mut(&c).unwrap() += i;
            } else {
                left_map.insert(c, i);
            }
            i *= 10;
        }
    }
    let mut i = 1;
    for c in right.trim().chars().rev() {
        if right_map.contains_key(&c) {
            *right_map.get_mut(&c).unwrap() += i;
        } else {
            right_map.insert(c, i);
        }
        i *= 10;
    }
    println!("{:?}", left_map);
    println!("{:?}", right_map);
    None
}
