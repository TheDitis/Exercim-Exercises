use regex::Regex;

const VOWEL_PATTERN: &str = "([aeiou]|xr|yt)";

pub fn translate(input: &str) -> String {
    let words = input.split(' ');
    words.map(translate_word).collect::<Vec<String>>().join(" ")
}

fn translate_word(input: &str) -> String {
    let qu_match = Regex::new("qu").unwrap().find(input);
    let first_vowel = Regex::new(VOWEL_PATTERN).unwrap().find(input).unwrap();
    let consonant_start = input[..first_vowel.start()].to_string();
    if first_vowel.start() == 0 {
        input[first_vowel.start()..].to_string() + "ay"
    } else if let Some(qu) = qu_match {
        format!("{}{}ay", &input[qu.end()..], &input[..qu.end()])
    } else {
        format!("{}{}ay", &input[first_vowel.start()..], consonant_start)
    }
}
