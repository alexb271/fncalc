use crate::branch::Branch;
use crate::expression::Expression;
use crate::function::{Function, FunctionCall, Return};
use crate::instruction::{Error, ErrorKind, Instruction, Print};
use crate::session;
use crate::token::{Operator, Parenthesis, Token};
use crate::while_loop::WhileLoop;
use crate::PI;

use pest::{iterators::Pair, iterators::Pairs, Parser};
use pest_derive::Parser;
use rust_decimal::Decimal;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct FnCalcParser;

pub fn parse(input: &str) -> Result<Vec<Instruction>, Error> {
    let parse_result = FnCalcParser::parse(Rule::start_symbol, &input);
    let parsed_content;

    match parse_result {
        Ok(mut content) => {
            parsed_content = content.next().unwrap();
        }
        Err(e) => {
            let pos = match e.line_col {
                pest::error::LineColLocation::Pos((_, column)) => column,
                pest::error::LineColLocation::Span((_, _), (column, _)) => column,
            };
            let err = Error::new(
                e.line().to_string(),
                pos.saturating_sub(1),
                ErrorKind::SyntaxError,
            );
            return Err(err);
        }
    }

    let mut result: Vec<Instruction> = Vec::new();

    for pair in parsed_content.into_inner() {
        match pair.as_rule() {
            Rule::expression => result.push(Instruction::Expression(build_expression(pair)?)),
            Rule::branch => result.push(Instruction::Branch(build_branch(pair)?)),
            Rule::while_loop => result.push(Instruction::WhileLoop(build_loop(pair)?)),
            Rule::function_return => result.push(Instruction::Return(build_function_return(pair)?)),
            Rule::print => result.push(Instruction::Print(build_print(pair)?)),
            Rule::function_definition => {
                let (name, function) = build_function_definition(pair)?;
                session::add_function(name, function);
            }
            Rule::EOI => (),
            _ => {
                dbg!(pair.as_rule());
                unreachable!();
            }
        }
    }

    Ok(result)
}

fn build_function_definition(function: Pair<Rule>) -> Result<(String, Function), Error> {
    let context = function.as_str().to_string();

    let mut function = function.into_inner();
    let name = function.next().unwrap().as_str().to_string();

    let mut body: Vec<Instruction> = Vec::new();
    let mut argument_names: Vec<String> = Vec::new();

    for pair in function {
        match pair.as_rule() {
            Rule::identifier => argument_names.push(pair.as_str().to_string()),
            Rule::expression => body.push(Instruction::Expression(build_expression(pair)?)),
            Rule::branch => body.push(Instruction::Branch(build_branch(pair)?)),
            Rule::while_loop => body.push(Instruction::WhileLoop(build_loop(pair)?)),
            Rule::function_return => body.push(Instruction::Return(build_function_return(pair)?)),
            Rule::print => body.push(Instruction::Print(build_print(pair)?)),
            _ => {
                dbg!(pair.as_rule());
                unreachable!();
            }
        }
    }

    Ok((name, Function::new(context, body, argument_names)))
}

fn build_function_return(function_return: Pair<Rule>) -> Result<Return, Error> {
    let context = function_return.as_str().to_string();
    let expr = function_return.into_inner().next().unwrap();

    Ok(Return::new(context, build_expression(expr)?))
}

fn build_print(print: Pair<Rule>) -> Result<Print, Error> {
    let expr = print.into_inner().next().unwrap();

    Ok(Print::new(build_expression(expr)?))
}

fn build_branch(control_flow: Pair<Rule>) -> Result<Branch, Error> {
    let context = control_flow.as_str().to_string();
    let mut control_flow = control_flow.into_inner();
    let condition = build_expression(control_flow.next().unwrap())?;

    let mut body: Vec<Instruction> = Vec::new();
    let mut body_else: Option<Vec<Instruction>> = None;

    for pair in control_flow {
        match pair.as_rule() {
            Rule::expression => body.push(Instruction::Expression(build_expression(pair)?)),
            Rule::branch => body.push(Instruction::Branch(build_branch(pair)?)),
            Rule::while_loop => body.push(Instruction::WhileLoop(build_loop(pair)?)),
            Rule::loop_break => body.push(Instruction::Break),
            Rule::function_return => body.push(Instruction::Return(build_function_return(pair)?)),
            Rule::print => body.push(Instruction::Print(build_print(pair)?)),
            Rule::branch_else => {
                body_else = Some(Vec::new());
                build_body(pair.into_inner(), body_else.as_mut().unwrap())?;
            }
            _ => unreachable!(),
        }
    }

    Ok(Branch::new(context, condition, body, body_else))
}

fn build_loop(control_flow: Pair<Rule>) -> Result<WhileLoop, Error> {
    let context = control_flow.as_str().to_string();
    let mut control_flow = control_flow.into_inner();
    let condition = build_expression(control_flow.next().unwrap())?;

    let mut body: Vec<Instruction> = Vec::new();
    build_body(control_flow, &mut body)?;

    Ok(WhileLoop::new(context, condition, body))
}

fn build_body(pairs: Pairs<Rule>, output: &mut Vec<Instruction>) -> Result<(), Error> {
    for pair in pairs {
        match pair.as_rule() {
            Rule::expression => output.push(Instruction::Expression(build_expression(pair)?)),
            Rule::branch => output.push(Instruction::Branch(build_branch(pair)?)),
            Rule::while_loop => output.push(Instruction::WhileLoop(build_loop(pair)?)),
            Rule::loop_break => output.push(Instruction::Break),
            Rule::function_return => output.push(Instruction::Return(build_function_return(pair)?)),
            Rule::print => output.push(Instruction::Print(build_print(pair)?)),
            Rule::branch_else => build_body(pair.into_inner(), output)?,
            _ => unreachable!(),
        }
    }

    Ok(())
}

fn build_expression(expression: Pair<Rule>) -> Result<Expression, Error> {
    let context = expression.as_str().to_string();
    let offset = expression.as_span().start();
    let mut output = Vec::new();
    tokenize_expression(expression, offset, &mut output)?;
    Ok(Expression::compile(output, context))
}

fn tokenize_expression(
    expression: Pair<Rule>,
    offset: usize,
    output: &mut Vec<Token>,
) -> Result<(), Error> {
    for pair in expression.into_inner() {
        let pos = pair.as_span().start() - offset;
        match pair.as_rule() {
            Rule::number => output.push(Token::new_val(
                pos,
                match Decimal::from_str_radix(pair.as_str(), 10) {
                    Ok(value) => value,
                    Err(_) => {
                        return Err(Error::new(
                            pair.as_str().to_string(),
                            pos,
                            ErrorKind::InvalidNumberLiteral,
                        ));
                    }
                },
            )),
            Rule::pi => output.push(Token::new_val(pos, PI)),
            Rule::identifier => output.push(Token::new_identifier(pos, pair.as_str().to_string())),
            Rule::add => output.push(Token::new_operator(pos, Operator::Add)),
            Rule::sub => output.push(Token::new_operator(pos, Operator::Sub)),
            Rule::mul => output.push(Token::new_operator(pos, Operator::Mult)),
            Rule::div => output.push(Token::new_operator(pos, Operator::Div)),
            Rule::modulo => output.push(Token::new_operator(pos, Operator::Mod)),
            Rule::pow => output.push(Token::new_operator(pos, Operator::Pow)),
            Rule::and => output.push(Token::new_operator(pos, Operator::And)),
            Rule::or => output.push(Token::new_operator(pos, Operator::Or)),
            Rule::less_than => output.push(Token::new_operator(pos, Operator::LessThan)),
            Rule::greater_than => output.push(Token::new_operator(pos, Operator::GreaterThan)),
            Rule::equal => output.push(Token::new_operator(pos, Operator::Equal)),
            Rule::not_equal => output.push(Token::new_operator(pos, Operator::NotEqual)),
            Rule::assign => output.push(Token::new_operator(pos, Operator::Assign)),
            Rule::neg => output.push(Token::new_operator(pos, Operator::Neg)),
            Rule::not => output.push(Token::new_operator(pos, Operator::Not)),

            Rule::sin => output.push(Token::new_operator(pos, Operator::Sin)),
            Rule::sind => output.push(Token::new_operator(pos, Operator::Sind)),
            Rule::asin => output.push(Token::new_operator(pos, Operator::Asin)),
            Rule::asind => output.push(Token::new_operator(pos, Operator::Asind)),

            Rule::cos => output.push(Token::new_operator(pos, Operator::Cos)),
            Rule::cosd => output.push(Token::new_operator(pos, Operator::Cosd)),
            Rule::acos => output.push(Token::new_operator(pos, Operator::Acos)),
            Rule::acosd => output.push(Token::new_operator(pos, Operator::Acosd)),

            Rule::tan => output.push(Token::new_operator(pos, Operator::Tan)),
            Rule::tand => output.push(Token::new_operator(pos, Operator::Tand)),
            Rule::atan => output.push(Token::new_operator(pos, Operator::Atan)),
            Rule::atand => output.push(Token::new_operator(pos, Operator::Atand)),

            Rule::ln => output.push(Token::new_operator(pos, Operator::Ln)),
            Rule::log => output.push(Token::new_operator(pos, Operator::Log)),
            Rule::abs => output.push(Token::new_operator(pos, Operator::Abs)),

            Rule::left_par => output.push(Token::new_parenthesis(pos, Parenthesis::Left)),
            Rule::right_par => output.push(Token::new_parenthesis(pos, Parenthesis::Right)),
            Rule::expression => tokenize_expression(pair, offset, output)?,
            Rule::function_call => output.push(build_function_call(pair, pos)?),
            Rule::EOI => (),
            _ => {
                dbg!(pair.as_rule());
                unreachable!()
            }
        }
    }
    Ok(())
}

fn build_function_call(function_call: Pair<Rule>, pos: usize) -> Result<Token, Error> {
    let context = function_call.as_str().to_string();

    let mut function_call = function_call.into_inner();
    let name = function_call.next().unwrap().as_str().to_string();

    let mut arguments: Vec<Expression> = Vec::new();
    for pair in function_call {
        match pair.as_rule() {
            Rule::expression => arguments.push(build_expression(pair)?),
            _ => unreachable!(),
        }
    }

    Ok(Token::new_function_call(
        pos,
        FunctionCall::new(context, name, arguments),
    ))
}
