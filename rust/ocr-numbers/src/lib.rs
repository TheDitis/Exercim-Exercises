use std::fmt::{Debug};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    validity_check(input)?;
    let split = group_number_strings(input);
    let parsed: Vec<String> = split.iter().map(|row| {
        row.iter().map(|n| convert_number(n.as_str())).collect()
    }).collect();
    Ok(parsed.join(","))
}

// returns Vec<Row<NumString>> where NumString is all the chars for a given number clumped together
fn group_number_strings(input: &str) -> Vec<Vec<String>> {
    // Get the individual char rows without the \n or empty separator lines
    let char_rows: Vec<String> = input.split('\n').enumerate().filter_map(|(i, s)| {
        let is_separating_line = i == 3 || (i > 4 && (i - 3) % 4 == 0);
        if is_separating_line { None } else { Some(String::from(s)) }
    }).collect();
    let row_width = char_rows.get(0).unwrap().len();
    char_rows.join("").chars().enumerate().fold(vec![], |mut v, (i, c)| {
        let row = i / (row_width * 3); // number of the current row of numbers
        if row >= v.len() { v.push(vec![]) }  // if i is at the start of a new row of numbers, add a new row
        let num_num = (i % row_width) / 3;  // number of the number in a given row that c belongs to to
        if num_num >= v[row].len() { v[row].push(String::new()) } // if the number has no entry yet, make one
        v[row][num_num].push(c); // add the char to the number string it belongs to, in the row it belongs to
        v
    })
}

fn convert_number(num_str: &str) -> char {
    match num_str {
        " _ | ||_|" => '0',
        "     |  |" => '1',
        " _  _||_ " => '2',
        " _  _| _|" => '3',
        "   |_|  |" => '4',
        " _ |_  _|" => '5',
        " _ |_ |_|" => '6',
        " _   |  |" => '7',
        " _ |_||_|" => '8',
        " _ |_| _|" => '9',
        _ => '?'
    }
}

fn validity_check(input: &str) -> Result<(), Error> {
    let split: Vec<&str> = input.split('\n').collect();
    if split.is_empty() || split[0].len() % 3 != 0 {
        Err(Error::InvalidColumnCount(split[0].len()))
    } else if split.len() % 4 != 0 {
        Err(Error::InvalidRowCount(split.len()))
    } else {
        Ok(())
    }
}