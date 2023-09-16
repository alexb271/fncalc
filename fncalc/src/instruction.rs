use crate::branch::Branch;
use crate::expression::Expression;
use crate::function::Return;
use crate::while_loop::WhileLoop;
use crate::Value;

#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
    SyntaxError,
    InvalidNumberLiteral,
    ZeroDivision,
    MathError,
    InvalidExponent,
    IdentifierNotFound,
    InvalidAssignment,
    MissingReturnValue,
    InvalidNumberOfArgument,
    IterationLimitReached,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorKind::SyntaxError => write!(f, "Syntax error"),
            ErrorKind::InvalidNumberLiteral => write!(f, "Invalid number"),
            ErrorKind::ZeroDivision => write!(f, "Division by zero"),
            ErrorKind::MathError => write!(f, "Math error"),
            ErrorKind::InvalidExponent => write!(f, "Invalid exponent"),
            ErrorKind::IdentifierNotFound => write!(f, "Identifier not found"),
            ErrorKind::InvalidAssignment => write!(f, "Invalid assignment"),
            ErrorKind::MissingReturnValue => write!(f, "Function did not return a value"),
            ErrorKind::IterationLimitReached => write!(f, "Maximum iteration count reached"),
            ErrorKind::InvalidNumberOfArgument => {
                write!(f, "Invalid number of arguments passed to function")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Error {
    context: String,
    pos: usize,
    kind: ErrorKind,
}

impl Error {
    pub fn new(context: String, pos: usize, kind: ErrorKind) -> Error {
        Error { context, pos, kind }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let ErrorKind::IterationLimitReached = self.kind() {
            write!(f, "Error: {}", self.kind())
        } else {
            let mut result = self.context.clone().replace("\n", " ");
            result.push('\n');

            for _ in 0..self.pos() {
                result.push_str(" ");
            }
            result.push_str("^\n");
            result.push_str(&format!("Error: {}", self.kind()));

            write!(f, "{result}")
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Clone, Copy)]
pub enum ReturnValue {
    Value(Value),
    Return(Value),
    None,
    Break,
}

impl ReturnValue {
    pub fn expect(self, msg: &str) -> Value {
        match self {
            ReturnValue::Value(v) | ReturnValue::Return(v) => v,
            _ => panic!("{msg}"),
        }
    }
}

pub type Result = std::result::Result<ReturnValue, Error>;

#[derive(Debug, Clone)]
pub enum Instruction {
    Expression(Expression),
    Branch(Branch),
    WhileLoop(WhileLoop),
    Return(Return),
    Print(Print),
    Break,
}

impl Instruction {
    pub fn exec(&self, output_stream: &mut String) -> Result {
        match self {
            Instruction::Expression(e) => e.exec(output_stream),
            Instruction::Branch(b) => b.exec(output_stream),
            Instruction::WhileLoop(l) => l.exec(output_stream),
            Instruction::Return(r) => r.exec(output_stream),
            Instruction::Print(p) => p.exec(output_stream),
            Instruction::Break => Ok(ReturnValue::Break),
        }
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Instruction::Expression(e) => write!(f, "{e}"),
            Instruction::Branch(b) => write!(f, "{b}"),
            Instruction::WhileLoop(l) => write!(f, "{l}"),
            Instruction::Return(r) => write!(f, "{r}"),
            Instruction::Print(p) => write!(f, "{p}"),
            Instruction::Break => write!(f, "break"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Print {
    expr: Expression,
}

impl Print {
    pub fn new(expr: Expression) -> Print {
        Print { expr }
    }

    pub fn exec(&self, output_stream: &mut String) -> Result {
        match self.expr.exec(output_stream) {
            Ok(output) => {
                output_stream.push_str(
                    &(crate::format_value(
                        output.expect("Expressions should always return a value on success"),
                    ) + "\n"),
                );
                Ok(ReturnValue::None)
            }
            Err(e) => Err(e),
        }
    }
}

impl std::fmt::Display for Print {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "print {}", self.expr)
    }
}
