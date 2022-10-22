
pub fn number(user_number: &str) -> Option<String> {
    // Remove all non-digit characters
    let mut filtered: String = user_number.chars()
        .filter(|c| c.is_ascii_digit())
        .collect();
    // Remove the country code if present
    if filtered.len() > 10 && filtered.chars().next()? == '1' {
        filtered.remove(0);
    }
    if filtered.len() == 10
        && between2and9(filtered.chars().next()?)
        && between2and9(filtered.chars().nth(3)?) {
        return Some(filtered)
    }
    None
}


fn between2and9(c: char) -> bool {
    let n = c.to_digit(10);
    n.is_some() && n.unwrap() >= 2 && n.unwrap() <= 9
}