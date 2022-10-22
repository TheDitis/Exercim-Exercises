// CONST Pattern matchers for each number
const ZERO: &str = " _ | ||_|";
//  _
// | |
// |_|";

const ONE: &str = "     |  |";
//  |
//  |";

const TWO: &str = " _  _||_ ";
//  _
//  _|
// |_ ";

const THREE: &str = " _  _| _|";
// _
// _|
// _|";

const FOUR: &str = "   |_|  |";
// |_|
//   |";

const FIVE: &str = " _ |_  _|";
//  _
// |_
//  _|";

const SIX: &str = " _ |_ |_|";
//  _
// |_
// |_|";

const SEVEN: &str = " _   |  |";
 // _
 //  |
 //  |";

const EIGHT: &str = " _ |_||_|";
//  _
// |_|
// |_|";

const NINE: &str = " _ |_| _|";
//  _
// |_|
//  _|";

use std::fmt::{Debug};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    validity_check(input)?;
    let split = group(input);
    let parsed: Vec<String> = split.iter().map(|row| {
        row.iter().map(|n| convert_number(n.as_str())).collect()
    }).collect();
    Ok(parsed.join(","))
}

fn group(input: &str) -> Vec<Vec<String>> {
    // Get the individual char rows without the \n or empty seperator lines
    let char_rows: Vec<String> = input.split('\n').enumerate().filter_map(|(i, s)| {
        let is_separating_line = i == 3 || (i > 4 && (i - 3) % 4 == 0);
        if is_separating_line { None } else { Some(String::from(s)) }
    }).collect();
    let num_nums_per_row: usize = char_rows.get(0).unwrap().len() / 3;
    // Organize all chars for a given number into a flat string, and group those by row: Vec<Row<NumString>>
    char_rows.chunks(3).map(|row| {
        (0..num_nums_per_row *3).step_by(3).map(|i| {
            row.iter().map(|line| &line.as_str()[i..i+3]).collect()
        }).collect()
    }).collect()
}

fn convert_number(num_str: &str) -> char {
    match num_str {
        ZERO => '0',
        ONE => '1',
        TWO => '2',
        THREE => '3',
        FOUR => '4',
        FIVE => '5',
        SIX => '6',
        SEVEN => '7',
        EIGHT => '8',
        NINE => '9',
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