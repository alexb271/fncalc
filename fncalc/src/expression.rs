use crate::instruction::{self, ReturnValue};
use crate::session;
use crate::token::{Operator, Parenthesis, Token, TokenKind};
use crate::{Value, NEGATIVE_ONE, ONE, ZERO};
use rust_decimal::prelude::*;

#[derive(Debug, Clone)]
pub struct Expression {
    context: String,
    tokens: Vec<Token>,
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.context)
    }
}

enum Operand<'a> {
    Value(Value),
    Identifier(&'a str, usize),
}

impl<'a> Operand<'a> {
    fn get_value(&self, context: &'a str) -> Result<Value, instruction::Error> {
        match self {
            Operand::Value(val) => Ok(*val),
            Operand::Identifier(id, pos) => match session::get_variable(id) {
                Some(val) => Ok(val),
                None => {
                    return Err(instruction::Error::new(
                        context.to_string(),
                        *pos,
                        instruction::ErrorKind::IdentifierNotFound,
                    ));
                }
            },
        }
    }
}

impl Expression {
    pub fn exec(&self, output_stream: &mut String) -> instruction::Result {
        let mut stack: Vec<Operand> = Vec::new();

        for token in &self.tokens {
            match token.kind() {
                TokenKind::Value(val) => stack.push(Operand::Value(val.clone())),
                TokenKind::Operator(op) => match op {
                    Operator::Add
                    | Operator::Sub
                    | Operator::Div
                    | Operator::Mult
                    | Operator::Mod
                    | Operator::Pow
                    | Operator::And
                    | Operator::Or
                    | Operator::LessThan
                    | Operator::GreaterThan
                    | Operator::Equal
                    | Operator::NotEqual => {
                        Expression::binary_operation(&mut stack, *op, &self.context, token.pos())?
                    }
                    Operator::Neg
                    | Operator::Not
                    | Operator::Sin
                    | Operator::Sind
                    | Operator::Asin
                    | Operator::Asind
                    | Operator::Cos
                    | Operator::Cosd
                    | Operator::Acos
                    | Operator::Acosd
                    | Operator::Tan
                    | Operator::Tand
                    | Operator::Atan
                    | Operator::Atand
                    | Operator::Ln
                    | Operator::Log
                    | Operator::Abs => {
                        Expression::unary_operation(&mut stack, *op, &self.context, token.pos())?
                    }
                    Operator::Assign => {
                        Expression::assignment_operation(&mut stack, &self.context, token.pos())?
                    }
                }
                TokenKind::Identifier(id) => stack.push(Operand::Identifier(&id, token.pos())),
                TokenKind::FunctionCall(f) => {
                    match session::call_function(f, token.pos(), output_stream) {
                        Ok(output) => match output {
                            ReturnValue::Value(value) => stack.push(Operand::Value(value)),
                            ReturnValue::None => {
                                return Err(instruction::Error::new(
                                    self.context.clone(),
                                    token.pos(),
                                    instruction::ErrorKind::MissingReturnValue,
                                ));
                            }
                            _ => unreachable!(
                                "Function calls should not return Break or Return types"
                            ),
                        },
                        Err(e) => return Err(e),
                    }
                }
                _ => unreachable!(),
            }
        }

        match stack.pop().unwrap().get_value(&self.context) {
            Ok(value) => Ok(ReturnValue::Value(value)),
            Err(e) => Err(e),
        }
    }

    fn assignment_operation(
        stack: &mut Vec<Operand>,
        context: &str,
        pos: usize,
    ) -> Result<(), instruction::Error> {
        let rhs = stack.pop().unwrap().get_value(context)?;
        let lhs = stack.pop().unwrap();

        match lhs {
            Operand::Value(_) => Err(instruction::Error::new(
                context.to_string(),
                pos,
                instruction::ErrorKind::InvalidAssignment,
            )),
            Operand::Identifier(id, _) => {
                session::set_variable(id, rhs);
                stack.push(lhs);
                Ok(())
            }
        }
    }

    fn binary_operation(
        stack: &mut Vec<Operand>,
        operator: Operator,
        context: &str,
        pos: usize,
    ) -> Result<(), instruction::Error> {
        let rhs = stack.pop().unwrap().get_value(context)?;
        let lhs = stack.pop().unwrap().get_value(context)?;

        let res = match operator {
            Operator::Add => Ok(lhs + rhs),
            Operator::Sub => Ok(lhs - rhs),
            Operator::Mult => Ok(lhs * rhs),
            Operator::Mod => Ok(lhs % rhs),
            Operator::Pow => Ok(lhs.math_pow(rhs, context, pos)?),
            Operator::Div => {
                if rhs != ZERO {
                    Ok(lhs / rhs)
                } else {
                    Err(instruction::ErrorKind::ZeroDivision)
                }
            }
            Operator::And => {
                if lhs != ZERO && rhs != ZERO {
                    Ok(ONE)
                } else {
                    Ok(ZERO)
                }
            }
            Operator::Or => {
                if lhs != ZERO || rhs != ZERO {
                    Ok(ONE)
                } else {
                    Ok(ZERO)
                }
            }
            Operator::LessThan => {
                if lhs < rhs {
                    Ok(ONE)
                } else {
                    Ok(ZERO)
                }
            }
            Operator::GreaterThan => {
                if lhs > rhs {
                    Ok(ONE)
                } else {
                    Ok(ZERO)
                }
            }
            Operator::Equal => {
                if lhs == rhs {
                    Ok(ONE)
                } else {
                    Ok(ZERO)
                }
            }
            Operator::NotEqual => {
                if lhs != rhs {
                    Ok(ONE)
                } else {
                    Ok(ZERO)
                }
            }
            Operator::Assign => {
                panic!(
                    "Do not use binary_operation for assignment, use assignment_operation instead"
                )
            }
            _ => panic!("Invalid operator for binary operation"),
        };

        match res {
            Ok(val) => {
                stack.push(Operand::Value(val));
                return Ok(());
            }
            Err(e) => {
                return Err(instruction::Error::new(context.to_string(), pos, e));
            }
        };
    }

    fn unary_operation(
        stack: &mut Vec<Operand>,
        operator: Operator,
        context: &str,
        pos: usize,
    ) -> Result<(), instruction::Error> {
        let operand = stack.pop().unwrap().get_value(context)?;

        let res = match operator {
            Operator::Neg => Ok(operand * NEGATIVE_ONE),
            Operator::Not => {
                if operand == ZERO {
                    Ok(ONE)
                } else {
                    Ok(ZERO)
                }
            }
            Operator::Sin => Ok(operand.trig_sin(context, pos)?),
            Operator::Sind => Ok(operand.to_radians().trig_sin(context, pos)?),
            Operator::Asin => Ok(operand.trig_asin(context, pos)?),
            Operator::Asind => Ok(operand.trig_asin(context, pos)?.to_degrees()),

            Operator::Cos => Ok(operand.trig_cos(context, pos)?),
            Operator::Cosd => Ok(operand.to_radians().trig_cos(context, pos)?),
            Operator::Acos => Ok(operand.trig_acos(context, pos)?),
            Operator::Acosd => Ok(operand.trig_acos(context, pos)?.to_degrees()),

            Operator::Tan => Ok(operand.trig_tan(context, pos)?),
            Operator::Tand => Ok(operand.trig_tan_deg(context, pos)?),
            Operator::Atan => Ok(operand.trig_atan(context, pos)?),
            Operator::Atand => Ok(operand.trig_atan(context, pos)?.to_degrees()),

            Operator::Ln => Ok(operand.math_ln(context, pos)?),
            Operator::Log => Ok(operand.math_log(context, pos)?),
            Operator::Abs => Ok(operand.abs()),
            _ => panic!("Invalid operator for unary operation"),
        };

        match res {
            Ok(val) => {
                stack.push(Operand::Value(val));
                return Ok(());
            }
            Err(e) => {
                return Err(instruction::Error::new(context.to_string(), pos, e));
            }
        };
    }

    pub fn compile(input: Vec<Token>, context: String) -> Expression {
        let mut output = Vec::new();
        let mut stack = Vec::new();

        for token in input {
            match token.kind() {
                TokenKind::Value(_) | TokenKind::Identifier(_) | TokenKind::FunctionCall(_) => {
                    output.push(token);
                }
                TokenKind::Operator(_) => {
                    Expression::process_operator(token, &mut stack, &mut output);
                }
                TokenKind::Parenthesis(_) => {
                    Expression::process_parenthesis(token, &mut stack, &mut output);
                }
            }
        }

        while stack.len() > 0 {
            output.push(stack.pop().unwrap());
        }

        Expression {
            tokens: output,
            context,
        }
    }

    fn process_operator(token: Token, stack: &mut Vec<Token>, output: &mut Vec<Token>) {
        let (precedence, left_assoc) = match token.kind() {
            TokenKind::Operator(op) => (op.precedence(), op.is_left_associative()),
            _ => panic!("Token is not an operator"),
        };

        if let Some(top_of_stack) = stack.last() {
            match top_of_stack.kind() {
                TokenKind::Parenthesis(Parenthesis::Left) => stack.push(token),
                TokenKind::Operator(op) => {
                    if op.precedence() < precedence {
                        stack.push(token);
                    } else if op.precedence() > precedence {
                        output.push(stack.pop().unwrap());
                        Expression::process_operator(token, stack, output);
                    } else {
                        if left_assoc {
                            output.push(stack.pop().unwrap());
                        }
                        stack.push(token);
                    }
                }
                _ => unreachable!(),
            }
        } else {
            stack.push(token);
        }
    }

    fn process_parenthesis(token: Token, stack: &mut Vec<Token>, output: &mut Vec<Token>) {
        let par = match token.kind() {
            TokenKind::Parenthesis(p) => p,
            _ => panic!("Token is not a parenthesis"),
        };

        match par {
            Parenthesis::Left => stack.push(token),
            Parenthesis::Right => loop {
                let temp = stack.pop().unwrap();
                if let TokenKind::Parenthesis(Parenthesis::Left) = temp.kind() {
                    break;
                } else {
                    output.push(temp);
                }
            },
        }
    }
}

trait Maths {
    fn to_radians(&self) -> Self;
    fn to_degrees(&self) -> Self;

    fn trig_sin(&self, context: &str, pos: usize) -> Result<Decimal, instruction::Error>;
    fn trig_cos(&self, context: &str, pos: usize) -> Result<Decimal, instruction::Error>;
    fn trig_tan(&self, context: &str, pos: usize) -> Result<Decimal, instruction::Error>;
    fn trig_tan_deg(&self, context: &str, pos: usize) -> Result<Decimal, instruction::Error>;

    fn trig_asin(&self, context: &str, pos: usize) -> Result<Decimal, instruction::Error>;
    fn trig_acos(&self, context: &str, pos: usize) -> Result<Decimal, instruction::Error>;
    fn trig_atan(&self, context: &str, pos: usize) -> Result<Decimal, instruction::Error>;

    fn math_ln(&self, context: &str, pos: usize) -> Result<Decimal, instruction::Error>;
    fn math_log(&self, context: &str, pos: usize) -> Result<Decimal, instruction::Error>;
    fn math_pow(&self, rhs: Decimal, context: &str, pos: usize) -> Result<Decimal, instruction::Error>;
}

impl Maths for Decimal {
    fn to_radians(&self) -> Decimal {
        self * (Decimal::PI / Decimal::from_isize(180).unwrap())
    }

    fn to_degrees(&self) -> Decimal {
        self * (Decimal::from_isize(180).unwrap() / Decimal::PI)
    }

    fn trig_sin(&self, context: &str, pos: usize) -> Result<Decimal, instruction::Error> {
        match self.checked_sin() {
            Some(value) => Ok(value),
            None => Err(instruction::Error::new(
                context.to_string(),
                pos,
                instruction::ErrorKind::MathError,
            )),
        }
    }

    fn trig_cos(&self, context: &str, pos: usize) -> Result<Decimal, instruction::Error> {
        match self.checked_cos() {
            Some(value) => Ok(value),
            None => Err(instruction::Error::new(
                context.to_string(),
                pos,
                instruction::ErrorKind::MathError,
            )),
        }
    }

    fn trig_tan(&self, context: &str, pos: usize) -> Result<Decimal, instruction::Error> {
        if self.abs() % Decimal::HALF_PI == Decimal::ZERO {
            return Err(instruction::Error::new(
                context.to_string(),
                pos,
                instruction::ErrorKind::MathError,
            ));
        }

        match self.checked_tan() {
            Some(value) => Ok(value),
            None => Err(instruction::Error::new(
                context.to_string(),
                pos,
                instruction::ErrorKind::MathError,
            )),
        }
    }

    fn trig_tan_deg(&self, context: &str, pos: usize) -> Result<Decimal, instruction::Error> {
        if self.abs() % Decimal::from_isize(180).unwrap() == Decimal::from_isize(90).unwrap() {
            return Err(instruction::Error::new(
                context.to_string(),
                pos,
                instruction::ErrorKind::MathError,
            ));
        }

        match self.to_radians().checked_tan() {
            Some(value) => Ok(value),
            None => Err(instruction::Error::new(
                context.to_string(),
                pos,
                instruction::ErrorKind::MathError,
            )),
        }
    }

    fn trig_asin(&self, context: &str, pos: usize) -> Result<Decimal, instruction::Error> {
        match Decimal::from_f64(self.to_f64().unwrap().asin()) {
            Some(value) => Ok(value),
            None => Err(instruction::Error::new(
                context.to_string(),
                pos,
                instruction::ErrorKind::MathError,
            )),
        }
    }

    fn trig_acos(&self, context: &str, pos: usize) -> Result<Decimal, instruction::Error> {
        match Decimal::from_f64(self.to_f64().unwrap().acos()) {
            Some(value) => Ok(value),
            None => Err(instruction::Error::new(
                context.to_string(),
                pos,
                instruction::ErrorKind::MathError,
            )),
        }
    }

    fn trig_atan(&self, context: &str, pos: usize) -> Result<Decimal, instruction::Error> {
        match Decimal::from_f64(self.to_f64().unwrap().atan()) {
            Some(value) => Ok(value),
            None => Err(instruction::Error::new(
                context.to_string(),
                pos,
                instruction::ErrorKind::MathError,
            )),
        }
    }

    fn math_ln(&self, context: &str, pos: usize) -> Result<Decimal, instruction::Error> {
        match self.checked_ln() {
            Some(value) => Ok(value),
            None => Err(instruction::Error::new(
                context.to_string(),
                pos,
                instruction::ErrorKind::MathError,
            )),
        }
    }

    fn math_log(&self, context: &str, pos: usize) -> Result<Decimal, instruction::Error> {
        match self.checked_log10() {
            Some(value) => Ok(value),
            None => Err(instruction::Error::new(
                context.to_string(),
                pos,
                instruction::ErrorKind::MathError,
            )),
        }
    }

    fn math_pow(&self, rhs: Decimal, context: &str, pos: usize) -> Result<Decimal, instruction::Error> {
        match self.checked_powd(rhs) {
            Some(value) => Ok(value),
            None => Err(instruction::Error::new(
                context.to_string(),
                pos,
                instruction::ErrorKind::InvalidExponent,
            )),
        }
    }
}
