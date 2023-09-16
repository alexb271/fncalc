use crate::expression::Expression;
use crate::instruction::{self, Instruction, ReturnValue};
use crate::{LOOP_LIMIT, ZERO};

#[derive(Debug, Clone)]
pub struct WhileLoop {
    context: String,
    condition: Expression,
    body: Vec<Instruction>,
}

impl std::fmt::Display for WhileLoop {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.context)
    }
}

impl WhileLoop {
    pub fn new(context: String, condition: Expression, body: Vec<Instruction>) -> WhileLoop {
        WhileLoop {
            context,
            condition,
            body,
        }
    }

    pub fn exec(&self, output_stream: &mut String) -> instruction::Result {
        let mut result = None;
        let mut condition_result = match self.condition.exec(output_stream) {
            Ok(value) => value.expect("Expressions should always return a value on success"),
            Err(e) => return Err(e),
        };

        let mut loop_counter: usize = 0;

        'main_loop: while condition_result != ZERO {
            if loop_counter >= LOOP_LIMIT {
                return Err(instruction::Error::new(
                    String::new(),
                    0,
                    instruction::ErrorKind::IterationLimitReached,
                ));
            } else {
                loop_counter += 1;
            }

            for item in &self.body {
                match item.exec(output_stream) {
                    Ok(return_value) => match return_value {
                        ReturnValue::Value(value) => result = Some(value),
                        ReturnValue::Return(value) => {
                            return Ok(ReturnValue::Return(value));
                        }
                        ReturnValue::Break => {
                            break 'main_loop;
                        }
                        ReturnValue::None => (),
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            }

            condition_result = match self.condition.exec(output_stream) {
                Ok(value) => value.expect("Expressions should always return a value on success"),
                Err(e) => return Err(e),
            };
        }

        match result {
            Some(value) => Ok(ReturnValue::Value(value)),
            None => Ok(ReturnValue::None),
        }
    }
}
