use core::fmt;
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum TokenType {
    RBRACKET,
    LBRACKET,
    RBRACE,
    LBRACE,
    DOT,
    COMMA,
    PLUS,
    MINUS,
    SLASH,
    STAR,
    LET,
    AND,
    OR,
    BANG,
    EQUAL,
    BANGEQUAL,
    EQUALEQUAL,
    GREATER,
    LESS,
    GREATEREQUAL,
    LESSEQUAL,
    SEMICOLON,
    TRUE,
    FALSE,
    FOR,
    IF,
    ELSE,
    WHILE,
    RETURN,
    SUPER,
    NULL,
    STRING,
    NUMBER,
    IDENTIFIER,
    FN,
    CLASS,
    OUTPUT,
    EOF,
}

pub enum DataType {
    NUMBER(f64),
    TEXT(String),
    NULL,
}

pub struct Token {
    pub name: TokenType,
    pub lexeme: String,
    pub value: DataType,
    pub line_number: usize,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_str = match self {
            TokenType::RBRACKET => "RBRACKET",
            TokenType::LBRACKET => "LBRACKET",
            TokenType::RBRACE => "RBRACE",
            TokenType::LBRACE => "LBRACE",
            TokenType::DOT => "DOT",
            TokenType::COMMA => "COMMA",
            TokenType::PLUS => "PLUS",
            TokenType::MINUS => "MINUS",
            TokenType::SLASH => "SLASH",
            TokenType::STAR => "STAR",
            TokenType::LET => "LET",
            TokenType::AND => "AND",
            TokenType::OR => "OR",
            TokenType::BANG => "BANG",
            TokenType::EQUAL => "EQUAL",
            TokenType::BANGEQUAL => "BANGEQUAL",
            TokenType::EQUALEQUAL => "EQUALEQUAL",
            TokenType::GREATER => "GREATER",
            TokenType::LESS => "LESS",
            TokenType::GREATEREQUAL => "GREATEREQUAL",
            TokenType::LESSEQUAL => "LESSEQUAL",
            TokenType::SEMICOLON => "SEMICOLON",
            TokenType::TRUE => "TRUE",
            TokenType::FALSE => "FALSE",
            TokenType::FOR => "FOR",
            TokenType::IF => "IF",
            TokenType::ELSE => "ELSE",
            TokenType::WHILE => "WHILE",
            TokenType::RETURN => "RETURN",
            TokenType::SUPER => "SUPER",
            TokenType::NULL => "NULL",
            TokenType::STRING => "STRING",
            TokenType::NUMBER => "NUMBER",
            TokenType::IDENTIFIER => "IDENTIFIER",
            TokenType::FN => "FN",
            TokenType::CLASS => "CLASS",
            TokenType::OUTPUT => "OUTPUT",
            TokenType::EOF => "EOF",
        };
        write!(f, "{}", token_str)
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataType::NULL => write!(f, "null"),
            DataType::NUMBER(value) => write!(f, "{}", value),
            DataType::TEXT(value) => write!(f, "{}", value),
        }
    }
}

impl Token {
    pub fn to_string(&self) -> String {
        return format!(
            "{} {} {} {}",
            self.name, self.lexeme, self.value, self.line_number
        );
    }
}
