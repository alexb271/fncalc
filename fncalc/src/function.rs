#![allow(dead_code)]
use crate::expression::Expression;
use crate::instruction::{self, Instruction, ReturnValue};

#[derive(Debug, Clone)]
pub struct Function {
    context: String,
    body: Vec<Instruction>,
    argument_names: Vec<String>,
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.context)
    }
}

impl Function {
    pub fn new(context: String, body: Vec<Instruction>, argument_names: Vec<String>) -> Function {
        Function {
            context,
            body,
            argument_names,
        }
    }

    pub fn argument_names(&self) -> &Vec<String> {
        &self.argument_names
    }

    pub fn exec(&self, output_stream: &mut String) -> instruction::Result {
        let mut result = None;
        for item in &self.body {
            match item.exec(output_stream) {
                Ok(return_value) => match return_value {
                    ReturnValue::Value(value) => result = Some(value),
                    ReturnValue::Return(value) => {
                        return Ok(ReturnValue::Value(value));
                    }
                    ReturnValue::None => (),
                    ReturnValue::Break => {
                        unreachable!("Function calls should not return Break or Return type")
                    }
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
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    context: String,
    name: String,
    arguments: Vec<Expression>,
}

impl FunctionCall {
    pub fn new(context: String, name: String, arguments: Vec<Expression>) -> FunctionCall {
        FunctionCall {
            context,
            name,
            arguments,
        }
    }

    pub fn context(&self) -> &str {
        &self.context
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn arguments(&self) -> &Vec<Expression> {
        &self.arguments
    }
}

impl std::fmt::Display for FunctionCall {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.context)
    }
}

#[derive(Debug, Clone)]
pub struct Return {
    context: String,
    expr: Expression,
}

impl Return {
    pub fn new(context: String, expr: Expression) -> Return {
        Return { context, expr }
    }

    pub fn context(&self) -> &str {
        &self.context
    }

    pub fn exec(&self, output_stream: &mut String) -> instruction::Result {
        match self.expr.exec(output_stream) {
            Ok(output) => Ok(ReturnValue::Return(
                output.expect("Expressions should always return a value on success"),
            )),
            Err(e) => Err(e),
        }
    }
}

impl std::fmt::Display for Return {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.context)
    }
}
