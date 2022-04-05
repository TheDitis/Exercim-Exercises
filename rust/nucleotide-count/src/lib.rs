use std::collections::HashMap;

const NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !is_valid_nucleotide(&nucleotide) { return Err(nucleotide) }
    let mut count = 0;
    for n in dna.chars() {
        if !is_valid_nucleotide(&n) { return Err(n) }
        if nucleotide == n { count += 1 }
    }
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut output = HashMap::new();
    if let Some(bad) = dna.chars().find(|n| { !is_valid_nucleotide(n) }) {
        return Err(bad)
    }
    for n in NUCLEOTIDES {
        output.insert(n, count(n, dna).unwrap());
    }
    Ok(output)
}


fn is_valid_nucleotide(c: &char) -> bool { NUCLEOTIDES.contains(c) }