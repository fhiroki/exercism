#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut v = vec![];
    for op in inputs {
        match op {
            CalculatorInput::Value(n) => v.push(*n),
            _ => {
                let v2 = v.pop()?;
                let v1 = v.pop()?;
                let result = match op {
                    CalculatorInput::Add => v1 + v2,
                    CalculatorInput::Subtract => v1 - v2,
                    CalculatorInput::Multiply => v1 * v2,
                    CalculatorInput::Divide => v1 / v2,
                    _ => 0,
                };
                v.push(result);
            }
        }
    }

    if v.len() != 1 {
        return None;
    }
    Some(v[0])
}
