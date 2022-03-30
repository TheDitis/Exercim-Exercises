
#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut num_stack: Vec<i32> = Vec::new();
    for input in inputs {
        match input {
            CalculatorInput::Value(val) => num_stack.push(*val),
            operator => {
                let r = num_stack.pop()?;
                let l = num_stack.pop()?;
                match operator {
                    CalculatorInput::Add => num_stack.push(l + r),
                    CalculatorInput::Subtract => num_stack.push(l - r),
                    CalculatorInput::Multiply => num_stack.push(l * r),
                    CalculatorInput::Divide => num_stack.push(l / r),
                    _ => { return None }
                }
            }
        }
    }
    if num_stack.len() == 1 { Some(num_stack[0]) } else { None }

}