/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() { return None }
    let mut dist = 0;
    for i in 0..s1.len() {
        if s1.as_bytes()[i] != s2.as_bytes()[i] { dist += 1; }
    }
    Some(dist)
}


/// Alternative solution using chaining
pub fn hamming_distance_2(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() { return None }
    Some(s1.chars().enumerate()
        .filter(|&(i, c)| c != s2.chars().nth(i).unwrap())
        .count())
}