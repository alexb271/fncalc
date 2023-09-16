use crate::function::FunctionCall;
use crate::Value;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mult,
    Div,
    Pow,
    Mod,
    Neg,
    And,
    Or,
    Not,
    LessThan,
    GreaterThan,
    Equal,
    NotEqual,
    Assign,

    Sin,
    Sind,
    Asin,
    Asind,

    Cos,
    Cosd,
    Acos,
    Acosd,

    Tan,
    Tand,
    Atan,
    Atand,

    Ln,
    Log,
    Abs,
}

impl Operator {
    pub fn precedence(&self) -> i8 {
        match self {
            Operator::Add => 3,
            Operator::Sub => 3,
            Operator::Mult => 4,
            Operator::Div => 4,
            Operator::Mod => 4,
            Operator::Pow => 5,
            Operator::Neg => 6,
            Operator::And => 2,
            Operator::Or => 2,
            Operator::Not => 6,
            Operator::LessThan => 1,
            Operator::GreaterThan => 1,
            Operator::Equal => 1,
            Operator::NotEqual => 1,
            Operator::Assign => 0,

            Operator::Sin => 6,
            Operator::Sind => 6,
            Operator::Asin => 6,
            Operator::Asind => 6,

            Operator::Cos => 6,
            Operator::Cosd => 6,
            Operator::Acos => 6,
            Operator::Acosd => 6,

            Operator::Tan => 6,
            Operator::Tand => 6,
            Operator::Atan => 6,
            Operator::Atand => 6,

            Operator::Ln => 6,
            Operator::Log => 6,
            Operator::Abs => 6,
        }
    }

    pub fn is_left_associative(&self) -> bool {
        match self {
            Operator::Add => true,
            Operator::Sub => true,
            Operator::Mult => true,
            Operator::Div => true,
            Operator::Mod => true,
            Operator::Pow => true,
            Operator::And => true,
            Operator::Or => true,
            Operator::LessThan => true,
            Operator::GreaterThan => true,
            Operator::Equal => true,
            Operator::NotEqual => true,
            _ => false,
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Sub => write!(f, "-"),
            Operator::Mult => write!(f, "*"),
            Operator::Div => write!(f, "/"),
            Operator::Mod => write!(f, "%"),
            Operator::Pow => write!(f, "^"),
            Operator::Neg => write!(f, "-"),
            Operator::And => write!(f, "and"),
            Operator::Or => write!(f, "or"),
            Operator::Not => write!(f, "not"),
            Operator::LessThan => write!(f, "<"),
            Operator::GreaterThan => write!(f, ">"),
            Operator::Equal => write!(f, "=="),
            Operator::NotEqual => write!(f, "!="),
            Operator::Assign => write!(f, "="),

            Operator::Sin => write!(f, "sin"),
            Operator::Sind => write!(f, "sind"),
            Operator::Asin => write!(f, "asin"),
            Operator::Asind => write!(f, "asind"),

            Operator::Cos => write!(f, "cos"),
            Operator::Cosd => write!(f, "cosd"),
            Operator::Acos => write!(f, "acos"),
            Operator::Acosd => write!(f, "acosd"),

            Operator::Tan => write!(f, "tan"),
            Operator::Tand => write!(f, "tand"),
            Operator::Atan => write!(f, "atan"),
            Operator::Atand => write!(f, "atand"),

            Operator::Ln => write!(f, "ln"),
            Operator::Log => write!(f, "log"),
            Operator::Abs => write!(f, "abs"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Parenthesis {
    Left,
    Right,
}

impl fmt::Display for Parenthesis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Parenthesis::Left => write!(f, "("),
            Parenthesis::Right => write!(f, ")"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    Value(Value),
    Identifier(String),
    FunctionCall(FunctionCall),
    Operator(Operator),
    Parenthesis(Parenthesis),
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenKind::Value(x) => write!(f, "{x}"),
            TokenKind::Identifier(x) => write!(f, "{x}"),
            TokenKind::FunctionCall(x) => write!(f, "{x}"),
            TokenKind::Operator(x) => write!(f, "{x}"),
            TokenKind::Parenthesis(x) => write!(f, "{x}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pos: usize,
    kind: TokenKind,
}

impl Token {
    pub fn new_val(pos: usize, value: Value) -> Token {
        Token {
            pos,
            kind: TokenKind::Value(value),
        }
    }

    pub fn new_identifier(pos: usize, value: String) -> Token {
        Token {
            pos,
            kind: TokenKind::Identifier(value),
        }
    }

    pub fn new_function_call(pos: usize, value: FunctionCall) -> Token {
        Token {
            pos,
            kind: TokenKind::FunctionCall(value),
        }
    }

    pub fn new_operator(pos: usize, value: Operator) -> Token {
        Token {
            pos,
            kind: TokenKind::Operator(value),
        }
    }

    pub fn new_parenthesis(pos: usize, value: Parenthesis) -> Token {
        Token {
            pos,
            kind: TokenKind::Parenthesis(value),
        }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}
