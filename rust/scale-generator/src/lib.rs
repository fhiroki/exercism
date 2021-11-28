// You should change this.
//
// Depending on your implementation, there are a variety of potential errors
// which might occur. They aren't checked by the test suite in order to
// allow the greatest freedom of implementation, but real libraries should
// provide useful, descriptive errors so that downstream code can react
// appropriately.
//
// One common idiom is to define an Error enum which wraps all potential
// errors. Another common idiom is to use a helper type such as failure::Error
// which does more or less the same thing but automatically.
#[derive(Debug)]
pub struct Error;

pub struct Scale {
    diatonic: Vec<String>,
    pitches: Vec<String>,
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        let mut scale = Self::chromatic(tonic).unwrap();
        let mut i = 0;
        for c in intervals.chars() {
            scale.diatonic.push(scale.pitches[i].to_string());
            i += match c {
                'm' => 1,
                'M' => 2,
                'A' => 3,
                _ => 0,
            };
        }
        Ok(scale)
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        let pitches = match tonic {
            "C" | "a" | "G" | "D" | "A" | "E" | "B" | "F#" | "e" | "b" | "f#" | "c#" | "g#"
            | "d#" => Self::get_pitches(
                tonic,
                &[
                    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
                ],
            ),
            _ => Self::get_pitches(
                tonic,
                &[
                    "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
                ],
            ),
        };
        Ok(Self {
            diatonic: vec![],
            pitches,
        })
    }

    fn get_pitches(tonic: &str, scales: &[&str]) -> Vec<String> {
        let mut pitches: Vec<String> = scales.iter().map(|c| c.to_string()).collect();
        let idx = pitches
            .iter()
            .position(|c| c.to_lowercase() == tonic.to_lowercase())
            .unwrap();
        let v: Vec<String> = pitches.drain(0..idx).collect();
        pitches.extend(v);
        pitches
    }

    pub fn enumerate(&self) -> Vec<String> {
        if !self.diatonic.is_empty() {
            self.diatonic.to_vec()
        } else {
            self.pitches.to_vec()
        }
    }
}
