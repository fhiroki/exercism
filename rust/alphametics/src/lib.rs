use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let v: Vec<&str> = input.trim().split(" == ").collect();
    let left = v[0].split(" + ");
    let right = v[1];
    let mut left_map : HashMap<char, u64> = HashMap::new();
    let mut right_map : HashMap<char, u64> = HashMap::new();
    let mut char_set = HashSet::new();
    let mut leading_char_set = HashSet::new();

    for s in left {
        let mut i = 1;
        leading_char_set.insert(s.chars().nth(0).unwrap());
        for c in s.trim().chars().rev() {
            if left_map.contains_key(&c) {
                *left_map.get_mut(&c).unwrap() += i;
            } else {
                left_map.insert(c, i);
                char_set.insert(c);
            }
            i *= 10;
        }
    }

    let mut i = 1;
    leading_char_set.insert(right.chars().nth(0).unwrap());
    for c in right.trim().chars().rev() {
        if right_map.contains_key(&c) {
            *right_map.get_mut(&c).unwrap() += i;
        } else {
            right_map.insert(c, i);
            char_set.insert(c);
        }
        i *= 10;
    }

    let char_list : Vec<char> = char_set.into_iter().collect();

    for perm in (0u8..10u8).permutations(char_list.len()) {
        let map : HashMap<char, _> = char_list.clone().into_iter().zip(perm.into_iter()).collect();
        let mut is_ok = true;
        for (key, value) in map.iter() {
            if *value == 0 && leading_char_set.contains(key) {
                is_ok = false;
                break;
            }
        }
        if !is_ok {
            continue;
        }

        let mut left_sum = 0;
        let mut right_sum = 0;

        for (key, value) in left_map.iter() {
            left_sum += (map[key] as u64) * value;
        }
        for (key, value) in right_map.iter() {
            right_sum += (map[key] as u64) * value;
        }

        if left_sum == right_sum {
            return Some(map);
        }
    }

    None
}
