use regex::Regex;

pub fn abbreviate(phrase: &str) -> String {
    let mut output = String::new();
    // matches '-'/'_'/' ' followed by letter, or lowercase followed by uppercase
    let re = Regex::new(r"( |-|_)([A-Z]|[a-z])|([a-z][A-Z])").unwrap();
    let matches = re.find_iter(phrase);
    // if the first char is a letter, add it to the acronym!
    if !phrase.is_empty() && phrase.chars().next().unwrap().is_ascii_alphabetic() {
        output.push(phrase.chars().next().unwrap().to_ascii_uppercase())
    }
    // add the second character of each match (start of the word) to the acronym
    for m in matches {
        output.push(m.as_str().chars().nth(1).unwrap().to_ascii_uppercase())
    }

    output
}