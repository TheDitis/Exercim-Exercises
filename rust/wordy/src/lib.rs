use regex::Regex;

pub struct WordProblem;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

impl Operation {
    fn apply(&self, a: i32, b: i32) -> i32 {
        match self {
            Self::Add => a + b,
            Self::Subtract => a - b,
            Self::Multiply => a * b,
            Self::Divide => a / b,
            Self::Power => a.pow(b as u32),
        }
    }
}

impl TryFrom<&str> for Operation {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "plus" => Ok(Self::Add),
            "minus" => Ok(Self::Subtract),
            "multiplied" => Ok(Self::Multiply),
            "divided" => Ok(Self::Divide),
            "power" => Ok(Self::Power),
            _ => Err("Invalid Operation string".to_string())
        }
    }
}

#[derive(Debug)]
enum Element {
    Number(i32),
    Operation(Operation)
}

impl TryFrom<&str> for Element {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(v) = value.parse::<i32>() {
            Ok(Self::Number(v))
        } else if let Ok(op) = Operation::try_from(value) {
            Ok(Self::Operation(op))
        } else {
            Err("Invalid element string".to_string())
        }
    }
}

pub fn answer(command: &str) -> Option<i32> {
    if !command.starts_with("What is ") || !command.ends_with('?') { return None }

    let parsed_elements = parse_command(convert_exponential_statements(command).as_str())?;
    Some(calculate_answer(parsed_elements))
}

fn calculate_answer(mut ops: Vec<Element>) -> i32 {
    if let Element::Number(mut n) = ops.remove(0) {
        for pair in ops.chunks(2) {
            if let (Element::Operation(op), Element::Number(n2)) = (&pair[0], &pair[1]) {
                n = op.apply(n, *n2)
            }
        }
        n
    } else { 0 }
}

// converts command string into a vec of elements, performing validity checks along the way
fn parse_command(command: &str) -> Option<Vec<Element>> {
    let parsed_elements =  command.trim_start_matches("What is ").trim_end_matches('?')
        .split(' ')
        .filter(|&el| el != "by")  // we only need the first part of "multiply by" etc.
        .map(Element::try_from);
    if !parsed_elements.clone().all(|el| el.is_ok()) { return None } // if there was a parsing error
    let parsed_elements: Vec<Element> = parsed_elements.map(|el| el.unwrap()).collect();
    // if it doesn't both start and end with a number and have an operation between any 2 numbers, return None
    if !parsed_elements.iter().enumerate().all(|(i, el)| {
        (matches!(el, Element::Number(_)) && i % 2 == 0) || (matches!(el, Element::Operation(_)) && i % 2 == 1)
    }) { return None }
    if !matches!(parsed_elements.last().unwrap(), Element::Number(_)) { return None };
    Some(parsed_elements)
}

fn convert_exponential_statements(command: &str) -> String {
    static POW_PATTERN: &str = "raised to the \\d+(th|nd|st|rd) power";
    let matcher = Regex::new(POW_PATTERN).unwrap();
    let mut output = String::from(command);
    let matches = matcher.find_iter(command);
    for pow_match in matches {
        let sub = &output[pow_match.start()..pow_match.end()];
        let n = sub.chars().filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<i32>().unwrap();
        output = output.replace(sub, format!("power {}", n).as_str());
    }
    output
}