use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let data: Vec<String> = input.iter().map(|s| s.to_lowercase()).collect();
    let arc = Arc::new(data);
    let mut threads = vec![];

    for i in 0..worker_count {
        let data = arc.clone();
        let thread = thread::spawn(move || {
            let mut map = HashMap::new();
            let mut k = i;
            while k < data.len() {
                for c in data.get(k).unwrap().chars().filter(|c| c.is_alphabetic()) {
                    *map.entry(c).or_insert(0) += 1;
                }
                k += worker_count;
            }
            map
        });
        threads.push(thread);
    }

    let mut map = HashMap::new();
    for thread in threads {
        for (k, v) in thread.join().unwrap() {
            *map.entry(k).or_insert(0) += v;
        }
    }
    map
}
