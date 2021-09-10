// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut map = HashMap::new();
    for v in magazine {
        map.insert(v, true);
    }

    for v in note {
        if (map.contains_key(v)) {
            map.remove(v);
        } else {
            return false;
        }
    }

    true
}
