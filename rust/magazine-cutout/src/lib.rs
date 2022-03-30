use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mag_counts = word_counts(magazine);
    let note_counts = word_counts(note);
    note_counts.iter().all(|(k, &v)| {
        mag_counts.contains_key(format_word(k).as_str())
            && v <= *mag_counts.get(format_word(k).as_str()).unwrap()
    })
}


pub fn word_counts(words: &[&str]) -> HashMap<String, u32> {
    let mut counts: HashMap<String, u32> = HashMap::new();
    for &word in words {
        *counts.entry(format_word(word)).or_insert(0) += 1;
    }
    counts
}

pub fn format_word(word: &str) -> String {
    word.replace(&['(', ')', ',', '\"', '.', ';', ':', '\''][..], "")
}