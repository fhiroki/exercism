extern crate rand;
use rand::Rng;

pub struct Robot(String);

impl Robot {
    pub fn new() -> Self {
        Robot(gen_random_name())
    }

    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn reset_name(&mut self) {
        self.0 = gen_random_name();
    }
}

pub fn gen_random_name() -> String {
    let mut rng = rand::thread_rng();
    let mut name = String::from("");
    for _ in 0..2 {
        name += &rng.gen_range('A'..='Z').to_string();
    }
    name += &format!("{:03}", rng.gen_range(0..1000));
    name
}
