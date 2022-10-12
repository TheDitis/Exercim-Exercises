use itertools::Itertools;

pub fn encode(n: u64) -> String {
    let grouped = group_by_thousands(n);
    grouped.iter().enumerate()
        .map(|(i, group)| encode_chunk(group, (grouped.len() - 1 - i) as u8, i == 0 && group.len() == 1))
        .filter(|c| !c.is_empty())
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn group_by_thousands(n: u64) -> Vec<String> {
    let str_num = n.to_string();
    str_num.chars().rev().chunks(3).into_iter()
        .map(|chunk| chunk.collect_vec().iter().rev().collect::<String>())
        .collect_vec().iter().cloned()
        .rev()
        .collect()
}

fn encode_chunk(group: &str, place: u8, is_only: bool) -> String {
    let mut group = group.to_string();
    let mut output: Vec<String> = vec![];
    // Handle & remove hundreds place if it's not 0
    if group.len() == 3 && !group.starts_with('0') {
        output.push(format!("{} {}", number_name(&group.remove(0), 1), "hundred"))
    }
    if group.len() == 3 { group.remove(0); }
    // handle all 10's place strings if the tens place doesn't hold a 0
    if group.len() == 2 && !group.starts_with('0') {
        // handle weird teen name cases
        if group.starts_with('1') {
            output.push(parse_teen_range(group.as_str()))
        }
        // handle any other tens place from 20 through 90
        else {
            let mut rest = vec![number_name(&group.remove(0), 2)];
            if !group.starts_with('0') {
                rest.push(number_name(&group.remove(0), 1))
            }
            output.push(rest.join("-"));
        }
    }
    // or add the final digit name if the place is 0 and this is the only chunk, or the group doesn't end with 0
    else if (place == 0 && is_only) || !group.ends_with('0') {
        output.push(number_name(&group.pop().unwrap(), 1))
    }
    // ADD CHUNK LABEL (like 'thousand', 'million', 'hundred', etc.) if needed
    if place != 0 && !output.is_empty() {
        match chunk_label(place + 1).as_str() {
            "" => {},
            valid_label => { output.push(valid_label.to_string()); }
        }
    }
    output.join(" ")
}

pub fn number_name(n_char: &char, place: u8) -> String {
    match place {
        2 => match n_char {
            '0' => String::from(""),
            '2' => String::from("twenty"),
            '3' => String::from("thirty"),
            '4' => String::from("forty"),
            '5' => String::from("fifty"),
            '6' => String::from("sixty"),
            '7' => String::from("seventy"),
            '8' => String::from("eighty"),
            '9' => String::from("ninety"),
             _  => String::from("INVALID-CHARACTER")
        },
        1 => match n_char {
            '0' => String::from("zero"),
            '1' => String::from("one"),
            '2' => String::from("two"),
            '3' => String::from("three"),
            '4' => String::from("four"),
            '5' => String::from("five"),
            '6' => String::from("six"),
            '7' => String::from("seven"),
            '8' => String::from("eight"),
            '9' => String::from("nine"),
             _  => String::from("INVALID-CHARACTER")
        },
        _ => String::new()
    }
}

fn parse_teen_range(n: &str) -> String {
    if n.len() != 2 && n.starts_with('1') { return String::new() }
    match n.chars().last().unwrap() {
        '0' => String::from("ten"),
        '1' => String::from("eleven"),
        '2' => String::from("twelve"),
        '3' => String::from("thirteen"),
        '4' => String::from("fourteen"),
        '5' => String::from("fifteen"),
        '6' => String::from("sixteen"),
        '7' => String::from("seventeen"),
        '8' => String::from("eighteen"),
        '9' => String::from("nineteen"),
        _ => String::from("INVALID TEEN RANGE STRING")
    }
}

pub fn chunk_label(chunk_num: u8) -> String {
    match chunk_num {
        7 => String::from("quintillion"),
        6 => String::from("quadrillion"),
        5 => String::from("trillion"),
        4 => String::from("billion"),
        3 => String::from("million"),
        2 => String::from("thousand"),
        1 => String::from("hundred"),
        _ => String::new(),
    }
}