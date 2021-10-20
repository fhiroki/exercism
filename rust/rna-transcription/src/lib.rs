#[derive(Debug, PartialEq)]
pub struct Dna(String);

#[derive(Debug, PartialEq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        const DNA: &str = "GCTA";
        for (i, c) in dna.chars().enumerate() {
            if !DNA.contains(c) {
                return Err(i);
            }
        }
        Ok(Dna(dna.to_string()))
    }

    pub fn into_rna(self) -> Rna {
        Rna(self.0.chars().fold(String::from(""), |mut rna, c| {
            rna.push(match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => 'X',
            });
            rna
        }))
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        const RNA: &str = "CGAU";
        for (i, c) in rna.chars().enumerate() {
            if !RNA.contains(c) {
                return Err(i);
            }
        }
        Ok(Rna(rna.to_string()))
    }
}
