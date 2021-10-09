// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use itertools::sorted;

#[allow(clippy::new_without_default)]
pub struct School(HashMap<u32, Vec<String>>);

impl School {
    pub fn new() -> School {
        School(HashMap::new())
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        match self.0.entry(grade) {
            Entry::Occupied(mut students) => {
                students.get_mut().push(student.to_string());
            }
            Entry::Vacant(students) => {
                students.insert(vec![student.to_string()]);
            }
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        sorted(self.0.keys().map(|k| *k)).collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        match self.0.get(&grade) {
            Some(students) => sorted(students.to_vec()).collect(),
            None => vec![],
        }
    }
}
