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
    EQUAL,
    EQUALEQUAL,
    GREATER,
    LESS,
    GREATEREQUAL,
    LESSEQUAL,
    TRUE,
    FALSE,
    STRING,
    IDENTIFIER,
    FN,
    CLASS,
    EOF,
}

pub enum DataType {
    NUMBER(f64),
    TEXT(String),
    BOOL(bool),
    NULL,
}

pub struct Token {
    pub name: TokenType,
    pub lexeme: String,
    pub value: DataType,
    pub line_number: usize,
}

impl Token {
    pub fn to_string(&self) -> String {
        return format!("{}", self.lexeme);
    }
}
