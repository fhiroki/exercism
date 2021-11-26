use anyhow::Error;
use std::fs;

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// In the real world, it's common to use crates such as [`clap`] or
/// [`structopt`] to handle argument parsing, and of course doing so is
/// permitted in this exercise as well, though it may be somewhat overkill.
///
/// [`clap`]: https://crates.io/crates/clap
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
/// [`structopt`]: https://crates.io/crates/structopt
#[derive(Debug, Default)]
pub struct Flags {
    n: bool,
    l: bool,
    i: bool,
    v: bool,
    x: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let mut f = Flags::default();
        for flag in flags {
            match *flag {
                "-n" => f.n = true,
                "-l" => f.l = true,
                "-i" => f.i = true,
                "-v" => f.v = true,
                "-x" => f.x = true,
                _ => continue,
            }
        }
        f
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut res = vec![];

    for file in files {
        let mut match_contents = vec![];
        let content = fs::read_to_string(file)?;
        for (i, line) in content.lines().enumerate() {
            let mut prefix = String::from("");
            if files.len() > 1 {
                prefix += &format!("{}:", file);
            }
            if flags.n {
                prefix += &format!("{}:", i + 1)
            }

            let mut s = line.to_string();
            let mut pat = pattern.to_string();
            if flags.i {
                s = s.to_lowercase();
                pat = pat.to_lowercase();
            }

            if flags.x {
                if is_match_line(flags.v, s == pat) {
                    match_contents.push(prefix + line);
                }
            } else {
                if is_match_line(flags.v, s.contains(&pat)) {
                    match_contents.push(prefix + line);
                }
            }
        }

        if flags.l {
            if !match_contents.is_empty() {
                res.push(file.to_string());
            }
        } else {
            res.extend(match_contents);
        }
    }

    Ok(res)
}

fn is_match_line(v_flag: bool, is_match: bool) -> bool {
    !v_flag && is_match || v_flag && !is_match
}
