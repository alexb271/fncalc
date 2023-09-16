use crate::expression::Expression;
use crate::instruction::{self, Instruction, ReturnValue};
use crate::ZERO;

#[derive(Debug, Clone)]
pub struct Branch {
    context: String,
    condition: Expression,
    body: Vec<Instruction>,
    body_else: Option<Vec<Instruction>>,
}

impl std::fmt::Display for Branch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.context)
    }
}

impl Branch {
    pub fn new(
        context: String,
        condition: Expression,
        body: Vec<Instruction>,
        body_else: Option<Vec<Instruction>>,
    ) -> Branch {
        Branch {
            context,
            condition,
            body,
            body_else,
        }
    }

    pub fn exec(&self, output_stream: &mut String) -> instruction::Result {
        let condition_result = match self.condition.exec(output_stream) {
            Ok(value) => value.expect("Expressions should always return a value on success"),
            Err(e) => return Err(e),
        };

        if condition_result != ZERO {
            exec_body(&self.body, output_stream)
        } else if let Some(body_else) = self.body_else.as_ref() {
            exec_body(&body_else, output_stream)
        } else {
            Ok(ReturnValue::None)
        }
    }
}

fn exec_body(body: &Vec<Instruction>, output_stream: &mut String) -> instruction::Result {
    let mut result = None;
    for item in body {
        match item.exec(output_stream) {
            Ok(return_value) => match return_value {
                ReturnValue::Value(value) => result = Some(value),
                ReturnValue::Return(value) => {
                    return Ok(ReturnValue::Return(value));
                }
                ReturnValue::Break => {
                    return Ok(ReturnValue::Break);
                }
                ReturnValue::None => (),
            },
            Err(e) => {
                return Err(e);
            }
        }
    }
    match result {
        Some(value) => Ok(ReturnValue::Value(value)),
        None => Ok(ReturnValue::None),
    }
}
