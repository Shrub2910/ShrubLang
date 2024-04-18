use std::vec;

use crate::token;
use std::process;
use token::TokenType;

struct LexerData {
    characters: Vec<char>,
    tokens: Vec<token::Token>,
    current: usize,
    start: usize,
    line_number: usize,
}

pub fn scan_tokens(contents: &str) -> Vec<token::Token> {
    let characters: Vec<char> = contents.chars().collect();
    let mut tokens: Vec<token::Token> = Vec::new();
    let mut current: usize = 0;
    let mut start: usize = 0;
    let mut line_number: usize = 0;

    let mut lexer_data: LexerData = LexerData {
        characters: characters,
        tokens: tokens,
        current: current,
        start: start,
        line_number: line_number,
    };

    while !is_at_end(&mut lexer_data) {
        start = current;
        scan_token(&mut lexer_data);
    }

    lexer_data.tokens
}

fn scan_token(lexer_data: &mut LexerData) {
    let c: char = advance(lexer_data);
    match c {
        '(' => append_generic_token(TokenType::LBRACKET, lexer_data),
        ')' => append_generic_token(TokenType::RBRACKET, lexer_data),
        '{' => append_generic_token(TokenType::LBRACE, lexer_data),
        '}' => append_generic_token(TokenType::RBRACE, lexer_data),
        '.' => append_generic_token(TokenType::DOT, lexer_data),
        ',' => append_generic_token(TokenType::COMMA, lexer_data),
        '+' => append_generic_token(TokenType::PLUS, lexer_data),
        '-' => append_generic_token(TokenType::MINUS, lexer_data),
        '*' => append_generic_token(TokenType::STAR, lexer_data),
        '=' => {
            if char_match('=', lexer_data) {
                append_generic_token(TokenType::EQUALEQUAL, lexer_data);
            } else {
                append_generic_token(TokenType::EQUAL, lexer_data);
            }
        }

        '>' => {}

        _ => process::exit(1),
    }
}

fn append_generic_token(token_type: TokenType, lexer_data: &mut LexerData) {
    let text: &[char] = &lexer_data.characters[lexer_data.start..lexer_data.current];
    let subtext: String = text.iter().collect();
    let new_token: token::Token = token::Token {
        name: token_type,
        lexeme: subtext,
        value: token::DataType::NULL,
        line_number: lexer_data.line_number,
    };

    lexer_data.tokens.push(new_token);
}

fn advance(lexer_data: &mut LexerData) -> char {
    lexer_data.current += 1;
    lexer_data.characters[lexer_data.current - 1]
}

fn is_at_end(lexer_data: &mut LexerData) -> bool {
    lexer_data.current >= lexer_data.characters.len()
}

fn char_match(expected: char, lexer_data: &mut LexerData) -> bool {
    if is_at_end(lexer_data) {
        return false;
    }

    if expected != lexer_data.characters[lexer_data.current] {
        return false;
    }

    lexer_data.current += 1;
    return true;
}
