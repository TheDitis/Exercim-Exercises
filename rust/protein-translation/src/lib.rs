use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    map: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.map.get(codon).copied()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut output: Vec<&str> = vec![];
        let char_vec = rna.chars().collect::<Vec<char>>();
        for chunk in char_vec.chunks(3) {
            let codon = chunk.iter().collect::<String>();
            let protein = self.name_for(codon.as_str())?;
            if protein == "stop codon" { break }
            output.push(protein);
        }
        if !output.is_empty() { Some(output) } else { None }
    }
}

impl<'a> From<Vec<(&'a str, &'a str)>> for CodonsInfo<'a> {
    fn from(pairs: Vec<(&'a str, &'a str)>) -> Self {
        let mut map = HashMap::new();
        for (codon, protein) in pairs.into_iter() {
            map.insert(codon, protein);
        }
        CodonsInfo { map }
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo::from(pairs)
}
