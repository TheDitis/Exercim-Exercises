
///-------------------------------------------------------------------------------------------------
///  Common
///-------------------------------------------------------------------------------------------------

pub type Value = i32;
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

///-------------------------------------------------------------------------------------------------
///  Operation
///-------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operation {
    fn apply(&self, a: i32, b: i32) -> i32 {
        match self {
            Self::Add => a + b,
            Self::Subtract => a - b,
            Self::Multiply => a * b,
            Self::Divide => a / b,
        }
    }
}

impl TryFrom<&str> for Operation {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Subtract),
            "*" => Ok(Self::Multiply),
            "/" => Ok(Self::Divide),
            _ => Err("Invalid Operation string".to_string())
        }
    }
}

///-------------------------------------------------------------------------------------------------
///  StackOperation
///-------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum StackOperation {
    Duplicate,
    Drop,
    Swap,
    Over,
}

impl StackOperation {
    fn apply(&self, stack: &mut Vec<Value>) -> Result<(), Error> {
        let check_min_size = |min: usize| {
            if stack.len() < min { Err(Error::StackUnderflow) } else { Ok(()) }
        };
        match self {
            Self::Duplicate => {
                check_min_size(1)?;
                stack.push(*stack.last().unwrap());
            },
            Self::Drop => {
                check_min_size(1)?;
                stack.pop();
            },
            Self::Swap => {
                check_min_size(2)?;
                let (val2, val1) = (stack.pop().unwrap(), stack.pop().unwrap());
                stack.extend([val2, val1]);
            },
            Self::Over => {
                check_min_size(2)?;
                stack.push(*stack.get(stack.len() - 2).unwrap());
            }
        }
        Ok(())
    }
}

impl TryFrom<&str> for StackOperation {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase().as_str() {
            "dup" => Ok(Self::Duplicate),
            "drop" => Ok(Self::Drop),
            "swap" => Ok(Self::Swap),
            "over" => Ok(Self::Over),
            other => Err(format!("{} is not a valid stack operation", other)),
        }
    }
}

///-------------------------------------------------------------------------------------------------
///  Token
///-------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
enum Token {
    Value(i32),
    Operation(Operation),
    StackOperation(StackOperation),
    Variable(usize),
    StoreVariable,
}

///-------------------------------------------------------------------------------------------------
///  Variable
///-------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct Variable {
    name: String,
    commands: Vec<Token>
}

///-------------------------------------------------------------------------------------------------
///  Forth
///-------------------------------------------------------------------------------------------------

#[derive(Default)]
pub struct Forth {
    stack: Vec<Value>,
    vars:  Vec<Variable>,
}

impl Forth {
    pub fn new() -> Forth { Forth::default() }

    pub fn stack(&self) -> &[Value] { self.stack.as_slice() }

    pub fn eval(&mut self, input: &str) -> Result<(), Error> {
        // split input by space, and loop until we have used all of them:
        let mut input: Vec<&str> = input.split(' ').collect();
        while !input.is_empty() {
            // try parsing the next item (or series of items in case of var assignment) into a token
            match self.parse_token(input.first().unwrap(), &mut input)? {
                Token::StoreVariable => (),  // if it's store-var command, we just want to store, not apply
                t => self.apply_token(t)?,
            };
            input.remove(0);
        }
        Ok(())
    }

    fn create_variable(&mut self, input: &mut Vec<&str>) -> Result<String, Error> {
        if let Some(end_ind) = input.iter().position(|s| s.len() == 1 && s.starts_with(';')) {
            let (_, var_name, values) = (input[0], input[1].to_string(), input[2..end_ind].to_vec());
            // make sure name is valid (it's not a number)
            if var_name.chars().all(|c| c.is_ascii_digit()) { return Err(Error::InvalidWord) }
            // // parse & store variable
            let mut parsed_values: Vec<Token> = vec![];
            for &str_val in &values {
                parsed_values.push(self.parse_token(str_val, &mut [].to_vec())?);
            }
            self.vars.push(Variable { name: var_name.clone().to_ascii_lowercase(), commands: parsed_values } );
            input.splice(0..(2 + values.len()), []);  // remove all related tokens from input
            Ok(var_name)
        } else { Err(Error::InvalidWord) }
    }

    fn parse_token(&mut self, next: &str, feed: &mut Vec<&str>) -> Result<Token, Error> {
        // is a number
        if let Ok(n) = next.parse::<isize>() {
            Ok(Token::Value(n as i32))
        }
        // if it's a reference to a stored variable
        else if let Ok(var_index) = self.find_var_index(next.to_string()) {
            Ok(Token::Variable(var_index))
        }
        // if it's the start of an assignment statement
        else if next.len() == 1 && next.starts_with(':') {
            self.create_variable(feed)?;
            Ok(Token::StoreVariable)
        }
        // is an operation
        else if let Ok(op) = Operation::try_from(next) {
            Ok(Token::Operation(op))
        }
        // is a builtin stack manipulation operation keyword
        else if let Ok(stack_op) = StackOperation::try_from(next) {
            Ok(Token::StackOperation(stack_op))
        }
        // anything else is an error
        else { Err(Error::UnknownWord) }
    }

    fn apply_token(&mut self, token: Token) -> Result<(), Error> {
        match token {
            Token::Value(n) => self.stack.push( n),
            Token::Variable(var_index) => {
                for t in self.vars.get(var_index).unwrap().clone().commands {
                    self.apply_token(t.clone())?
                }
            },
            Token::Operation(op) => {
                if self.stack.len() < 2 { return Err(Error::StackUnderflow) }
                let (val2, val1) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
                if val2 == 0 && matches!(op, Operation::Divide) { return Err(Error::DivisionByZero) }
                self.stack.push(op.apply(val1, val2));
            },
            Token::StackOperation(stack_op) => {
                stack_op.apply(&mut self.stack)?;
            },
            Token::StoreVariable => {},
        }
        Ok(())
    }

    fn find_var_index(&self, var_name: String) -> Result<usize, Error> {
        for (i, var) in self.vars.iter().enumerate().rev() {
            if var.name.to_ascii_lowercase() == var_name.to_ascii_lowercase() {
                return Ok(i)
            }
        }
        Err(Error::UnknownWord)
    }

}
