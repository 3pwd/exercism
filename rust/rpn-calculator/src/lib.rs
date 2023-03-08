extern crate core;

#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate_one(operation: &CalculatorInput, operand1: i32, operand2: i32) -> i32 {
    match operation {
        CalculatorInput::Value(_value) => panic!("wrong operation"),
        CalculatorInput::Add => operand2 + operand1,
        CalculatorInput::Subtract => operand2 - operand1,
        CalculatorInput::Multiply => operand2 * operand1,
        CalculatorInput::Divide => operand2 / operand1,
    }
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();

    for input in inputs {
        match input {
            CalculatorInput::Value(value) => stack.push(*value),
            _ => {
                if stack.len() < 2 {
                    return None;
                }

                let operand1 = stack.pop()?;
                let operand2 = stack.pop()?;

                stack.push(evaluate_one(input, operand1, operand2))
            }
        }
    }

    if stack.len() > 1 {
        return None;
    }

    stack.pop()
}
