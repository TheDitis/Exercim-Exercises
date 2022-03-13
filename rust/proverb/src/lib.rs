pub fn build_proverb(list: &[&str]) -> String {
    let mut result = String::new();
    if list.len() == 0 { return result }
    for (i, word) in list[1..].iter().enumerate() {
        let prior_word = list[i];  // not i-1 because enumerated after slicing starting at 1
        result.push_str(format!("For want of a {} the {} was lost.\n", prior_word, word).as_str())
    }
    result.push_str(format!("And all for the want of a {}.", list[0]).as_str());
    result
}
