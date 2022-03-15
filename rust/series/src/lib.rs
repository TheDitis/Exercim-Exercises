pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut output: Vec<String> = vec!();
    if len > digits.len() { return output };
    for i in 0..digits.len() - len + 1 {
        output.push(digits[i..i+len].to_string());
    }
    output
}
