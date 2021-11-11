use regex::Regex;
use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    nums: Vec<Value>,
    map: HashMap<String, String>,
    useless: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

macro_rules! pop_item {
    ( $v:ident ) => {
        match $v.nums.pop() {
            Some(x) => x,
            None => return Err(Error::StackUnderflow),
        }
    };
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            nums: vec![],
            map: HashMap::new(),
            useless: vec![],
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.nums
    }

    pub fn set_definition(&mut self, input: &str) -> std::result::Result<String, Error> {
        let mut codes = input.to_lowercase().to_string();
        let re = Regex::new(r":\s(.*?)\s(.*?)\s;").unwrap();

        for cap in re.captures_iter(&input.to_lowercase()) {
            codes = codes.replace(&cap[0], "");

            if cap[1].chars().all(|c| c.is_numeric()) {
                return Err(Error::InvalidWord);
            }

            let left = cap[1].to_string();
            let def: Vec<&str> = cap[2].split_whitespace().collect();
            if def.len() == 2 && def[1] == "drop" {
                self.useless.push(left.to_owned());
            }

            let right = def.iter().fold(String::new(), |merged, w| {
                let w = match self.map.get(*w) {
                    Some(x) => {
                        if self.useless.contains(&w.to_string()) {
                            ""
                        } else {
                            x
                        }
                    }
                    None => *w,
                };
                merged + " " + w
            });

            if right.trim().is_empty() {
                self.useless.push(left.to_owned());
            }

            self.map.insert(left, right);
        }

        for (k, v) in self.map.iter() {
            codes = codes.replace(k, v);
        }

        Ok(codes)
    }

    fn basic_operation(&mut self, op: &str) -> Result {
        let v2 = pop_item!(self);
        let v1 = pop_item!(self);
        let res = match op {
            "+" => v1 + v2,
            "-" => v1 - v2,
            "*" => v1 * v2,
            "/" => {
                if v2 == 0 {
                    return Err(Error::DivisionByZero);
                }
                v1 / v2
            }
            _ => return Err(Error::InvalidWord),
        };
        self.nums.push(res);
        Ok(())
    }

    fn dup(&mut self) -> Result {
        match self.nums.last() {
            Some(&x) => {
                self.nums.push(x);
                Ok(())
            }
            None => Err(Error::StackUnderflow),
        }
    }

    fn drop(&mut self) -> Result {
        match self.nums.pop() {
            Some(_) => Ok(()),
            None => Err(Error::StackUnderflow),
        }
    }

    fn swap(&mut self) -> Result {
        let n = self.nums.len();
        if n < 2 {
            return Err(Error::StackUnderflow);
        }
        self.nums.swap(n - 1, n - 2);
        Ok(())
    }

    fn over(&mut self) -> Result {
        let n = self.nums.len();
        if n < 2 {
            return Err(Error::StackUnderflow);
        }
        self.nums.push(self.nums[n - 2]);
        Ok(())
    }

    pub fn eval(&mut self, input: &str) -> Result {
        if input.matches(':').count() != input.matches(';').count() {
            return Err(Error::InvalidWord);
        }
        let codes = self.set_definition(&input.to_lowercase())?;
        for s in codes.split_whitespace() {
            match s.parse::<Value>() {
                Ok(x) => self.nums.push(x),
                Err(_) => match s.as_ref() {
                    "+" | "-" | "*" | "/" => self.basic_operation(&s)?,
                    "dup" => self.dup()?,
                    "drop" => self.drop()?,
                    "swap" => self.swap()?,
                    "over" => self.over()?,
                    _ => return Err(Error::UnknownWord),
                },
            }
        }
        Ok(())
    }
}
