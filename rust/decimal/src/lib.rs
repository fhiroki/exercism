use num_bigint::{BigInt, ToBigInt};
use std::cmp::{max, Ordering};
use std::ops::{Add, Mul, Sub};

#[derive(Eq, Debug, Clone)]
pub struct Decimal {
    num: BigInt,
    pos: u32,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let pos = input.chars().rev().position(|c| c == '.').unwrap_or(0) as u32;
        let num = BigInt::parse_bytes(input.replace(".", "").as_bytes(), 10).unwrap();
        Some(Self { num, pos }.align())
    }

    fn calc(&self, other: &Self, op: fn(BigInt, BigInt) -> BigInt) -> BigInt {
        let mut p = 1.to_bigint().unwrap();
        for _ in 0..(self.pos as i32 - other.pos as i32).abs() {
            p *= 10;
        }
        if self.pos <= other.pos {
            op(self.num.clone() * p, other.num.clone())
        } else {
            op(self.num.clone(), other.num.clone() * p)
        }
    }

    fn align(self) -> Self {
        let pos_non_zero = self
            .num
            .to_str_radix(10)
            .chars()
            .rev()
            .position(|c| c != '0')
            .unwrap_or(0) as u32;
        let pos = u32::checked_sub(self.pos, pos_non_zero).unwrap_or(0);
        let mut p = 1.to_bigint().unwrap();
        if self.pos != 0 {
            for _ in 0..pos_non_zero {
                p *= 10;
            }
        }
        let num = self.num / p;
        Self { num, pos }
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    fn lt(&self, other: &Self) -> bool {
        let mut p = 1.to_bigint().unwrap();
        for _ in 0..(self.pos as i32 - other.pos as i32).abs() {
            p *= 10;
        }
        if self.pos <= other.pos {
            self.num.clone() * p < other.num
        } else {
            self.num < other.num.clone() * p
        }
    }

    fn gt(&self, other: &Self) -> bool {
        !self.lt(other)
    }
}

impl Ord for Decimal {
    fn cmp(&self, other: &Self) -> Ordering {
        self.num.cmp(&other.num)
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num
    }
}

impl Add for Decimal {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            num: self.calc(&other, |x, y| x + y),
            pos: max(self.pos, other.pos),
        }
        .align()
    }
}

impl Sub for Decimal {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            num: self.calc(&other, |x, y| x - y),
            pos: max(self.pos, other.pos),
        }
        .align()
    }
}

impl Mul for Decimal {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            num: self.calc(&other, |x, y| x * y),
            pos: max(self.pos, other.pos),
        }
        .align()
    }
}
