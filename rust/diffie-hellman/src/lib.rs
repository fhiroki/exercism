extern crate rand;
use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    mod_pow(b_pub, a, p)
}

fn mod_pow(x: u64, e: u64, m: u64) -> u64{
    let mut result: u64 = 1;
    let mut base = x % m;
    let mut exponent = e;
    while exponent > 0{
        if exponent % 2 == 1 {
            result = (result * base) % m;
        }
        exponent = exponent / 2;
        base = (base * base) % m;
    }
    result
}
