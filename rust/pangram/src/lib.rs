use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut present_chars = HashSet::new();
    for c in sentence.chars() {
        if c.is_ascii_alphabetic() {
            present_chars.insert(c.to_lowercase().nth(0));
        }
    }
    present_chars.len() == 26
}
