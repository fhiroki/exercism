use std::collections::HashMap;

const DNA: &str = "ACGT";

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let map = nucleotide_counts(dna)?;
    map.get(&nucleotide).map(|n| *n).ok_or(nucleotide)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map = HashMap::new();
    for c in DNA.chars() {
        map.insert(c, 0);
    }
    for c in dna.chars() {
        if !DNA.contains(c) {
            return Err(c);
        }
        *map.get_mut(&c).unwrap() += 1;
    }
    Ok(map)
}
