#[derive(Debug, PartialEq)]
pub struct Dna {
    nucleotides: Vec<char>
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    nucleotides: Vec<char>
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let nucleotides = dna.to_string();
        for (i, c) in nucleotides.chars().enumerate() {
            if !['A', 'C', 'G', 'T'].contains(&c) {
                return Err(i)
            }
        }
        Ok(Dna { nucleotides: nucleotides.chars().collect() })
    }

    pub fn into_rna(self) -> Rna {
        Rna::from(self)
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let nucleotides = rna.to_string();
        for (i, c) in nucleotides.chars().enumerate() {
            if !['A', 'C', 'G', 'U'].contains(&c) {
                return Err(i)
            }
        }
        Ok(Rna { nucleotides: nucleotides.chars().collect() })
    }
}

impl From<Dna> for Rna {
    fn from(dna: Dna) -> Self {
        let nucleotides: Vec<char> = dna.nucleotides.iter()
            .map(|c| {
                match c {
                    'G' => 'C',
                    'C' => 'G',
                    'T' => 'A',
                    'A' => 'U',
                     _  => '?',
                }
            }).collect();
        Rna { nucleotides }
    }
}