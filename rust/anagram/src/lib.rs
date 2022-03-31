use std::collections::{HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut output = HashSet::new();
    let word = word.to_lowercase();

    for &other_word in possible_anagrams {
        let word2 = other_word.to_lowercase();
        if word != word2 && sort_string(&word) == sort_string(&word2) {
            output.insert(other_word);
        }
    }
    output
}


fn sort_string(s: &str) -> String {
    let mut sorted = s.to_string().chars().collect::<Vec<char>>();
    sorted.sort_unstable();
    sorted.iter().collect()
}