pub fn encode(source: &str) -> String {
    let mut output = String::new();
    let mut i = 0;
    while i < source.len() {
        let cur = source.chars().nth(i).unwrap();
        let run: String = source[i..].chars().take_while(|&c| { c == cur }).collect();
        let count = run.len();
        i += count;
        output.push_str(encode_run(cur, count as u32).as_str());
    }
    output
}

pub fn decode(source: &str) -> String {
    let mut output = String::new();
    let mut i = 0;
    while i < source.len() {
        let num_str: String = source[i..].chars().take_while(|c| c.is_ascii_digit()).collect();
        let c = source.chars().nth(i + num_str.len()).unwrap();
        let num: u32 = num_str.parse().unwrap_or(1);
        output.push_str(make_run(c, num).as_str());
        i += num_str.len() + 1;
    }
    output
}


fn encode_run(char: char, count: u32) -> String {
    if count > 1 { format!("{}{}", count, char) } else { char.to_string() }
}

fn make_run(char: char, count: u32) -> String {
    char.to_string().repeat(count as usize)
}