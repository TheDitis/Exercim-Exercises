use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let delimiters = &[' ', '\t', '\n', ',', '.', ':'][..];

    words.split(delimiters).filter(|&w| !w.is_empty())
        .fold(HashMap::new(), |mut acc, word| {
            *acc.entry(trim_word(word).to_ascii_lowercase()).or_insert(0) += 1;
            acc
        })
}


pub fn trim_word(word: &str) -> String {
    let valid_start_end = |c: &char| !c.is_ascii_alphabetic() && !c.is_ascii_digit();
    let mut chars: Vec<char> = word.chars().skip_while(valid_start_end).collect();
    chars = chars.iter().rev().copied().skip_while(valid_start_end).collect();
    chars.iter().rev().collect()
}