/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn: String = filter_chars(isbn).unwrap_or_else(|| "".to_string());
    if isbn.len() != 10 { return false }
    let mut multipliers = (1..=10).collect::<Vec<u32>>();
    isbn.chars().enumerate().fold(0, |sum, (i, c)| {
        let n = if i == 9 && is_x(c) { 10 } else { c.to_digit(10).unwrap() };  // convert the check digit x to 10 if needed
        sum + (n * multipliers.pop().unwrap())
    }) % 11 == 0
}


fn filter_chars(isbn: &str) -> Option<String> {
    // filter out any non-digits and non-X chars
    let only_digits_and_xs: String = isbn.chars()
        .filter(|&c| { c.is_ascii_digit() || is_x(c) }).collect();
    // split body and check digit & filter out Xs that may be in the body
    let check_digit = only_digits_and_xs.chars().last()?;
    let body: String = only_digits_and_xs[..only_digits_and_xs.len() - 1].chars()
        .filter(|c| { c.is_ascii_digit() }).collect();
    Some(body + check_digit.to_string().as_str())  // return filtered body with check digit add back on
}

fn is_x(c: char) -> bool { c == 'x' || c == 'X' }