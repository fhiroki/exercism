// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let mut rate = (221.0 * speed as f64);
    if speed >= 9 {
        rate *= 0.77;
    } else if speed >= 5 {
        rate *= 0.9;
    }
    return rate;
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
