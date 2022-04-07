use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut char_set: HashSet<char> = HashSet::new();
    let candidate = candidate.to_lowercase();
    for c in candidate.chars() {
        if c == '-' || c == ' ' { continue; }
        if !char_set.insert(c) { return false; }
    }
    true
}
