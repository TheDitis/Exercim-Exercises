use std::collections::HashMap;
use regex::Regex;

enum Item {
    Number(u32),
    Letter(char),
}

struct AlphameticsPuzzle {
    char_num_map: HashMap<char, u8>,
    addends: Vec<String>,
    result: String,
    max: Vec<u8>,
}


impl AlphameticsPuzzle {
    pub fn try_from(input: &str) -> Option<Self> {
        let (addends, result) = split_input(input)?;
        Some(AlphameticsPuzzle {
            char_num_map: HashMap::new(),
            max: max_sum_digits(&addends),
            addends,
            result,
        })
    }

    pub fn solve(&mut self) -> Option<HashMap<char, u8>> {
        for n in self.max.n {

        }
    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut puzzle = AlphameticsPuzzle::try_from(input)?;
    puzzle.solve()
    // eprintln!("input = {:?}", input);
    // let mut output: HashMap<char, u8> = HashMap::new();
    // let max_digits = max_sum_digits(addends);
    // eprintln!("addends = {:?}", addends);
    // eprintln!("result = {:?}", result);
    // if result.len() > addends.iter().map(|v| v.len()).max()? {
    //     output.insert(result.chars().next().unwrap(), 1);
    // }
    // eprintln!("output = {:?}", output);
    // None
}

fn split_input(input: &str) -> Option<(Vec<String>, String)> {
    let split = Regex::new(r" == | \+ ").unwrap().split(input).collect::<Vec<&str>>();
    if let [addends @ .., result] = split.as_slice() {
        Some((addends.iter().map(|&v| String::from(v)).collect(), result.to_string()))
    } else {
        println!("None from split_input");
        None
    }
}

fn max_sum_digits(additor_strs: &Vec<String>) -> Vec<u8> {
    additor_strs.iter()
        .map(|s| s.chars().map(|_| '9').collect::<String>().parse::<u32>().unwrap())
        .sum::<u32>()
        .to_string()
        .as_bytes()
        .to_vec()
}