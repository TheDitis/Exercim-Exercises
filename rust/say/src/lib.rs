
pub fn encode(n: u64) -> String {
    let mut output = String::new();
    let grouped = group_by_thousands(n);
    let mut group_num = grouped.len();
    for group in grouped {
        output.extend(encode_chunk(group, group_num as u8).chars().into_iter());
        group_num -= 1;
    }

    output
}

pub fn encode_chunk(chunk: Vec<char>, chunk_num: u8) -> String {
    let mut output = String::new();
    let mut place_num = chunk.len();
    for (i, c) in chunk.iter().enumerate() {
        output.push_str(number_name(c, place_num as u8).as_str());
        place_num -= 1;
    }
    output.push_str(chunk_label(chunk_num).as_str());
    output
}


pub fn group_by_thousands(n: u64) -> Vec<Vec<char>> {
    let str_num = n.to_string();
    let chars = str_num.chars().rev().collect::<Vec<char>>();
    println!("{:?}", chars);
    let mut output: Vec<Vec<char>> = vec![];
    for chunk in chars[..].chunks(3) {
        output.insert(0, chunk.iter().rev().copied().collect());
    }
    output
}

pub fn number_name(n_char: &char, place: u8) -> String {
    match place {
        2 => match n_char {
            '0' => String::from(""),
            '1' => String::from("UNHANDLED-TEEN-RANGE"),
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
            '0' => String::from(""),
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

pub fn chunk_label(chunk_num: u8) -> String {
    match chunk_num {
        7 => String::from(" quintillion"),
        6 => String::from(" quadrillion"),
        5 => String::from(" trillion"),
        4 => String::from(" billion"),
        3 => String::from(" million"),
        2 => String::from(" thousand"),
        _ => String::from(""),
    }
}