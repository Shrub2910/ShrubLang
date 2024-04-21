use crate::shrub_error;
use crate::shrub_error::SyntaxError;
use crate::token;
use std::collections::HashMap;
use token::TokenType;

struct LexerData {
    characters: Vec<char>,
    tokens: Vec<token::Token>,
    current: usize,
    start: usize,
    line_number: usize,
    key_words: HashMap<String, TokenType>,
}

pub fn scan_tokens(contents: &str) -> Result<Vec<token::Token>, shrub_error::SyntaxError> {
    let characters: Vec<char> = contents.chars().collect();
    let tokens: Vec<token::Token> = Vec::new();
    let current: usize = 0;
    let start: usize = 0;
    let line_number: usize = 0;
    let mut key_words: HashMap<String, TokenType> = HashMap::new();

    populate_key_words(&mut key_words);

    let mut lexer_data: LexerData = LexerData {
        characters: characters,
        tokens: tokens,
        current: current,
        start: start,
        line_number: line_number,
        key_words: key_words,
    };

    while !is_at_end(&mut lexer_data) {
        lexer_data.start = lexer_data.current;
        match scan_token(&mut lexer_data) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
    }

    lexer_data.tokens.push(token::Token {
        name: TokenType::EOF,
        lexeme: String::new(),
        value: token::DataType::NULL,
        line_number: lexer_data.line_number,
    });

    Ok(lexer_data.tokens)
}

fn populate_key_words(keywords: &mut HashMap<String, TokenType>) {
    keywords.insert("and".to_string(), TokenType::AND);
    keywords.insert("or".to_string(), TokenType::OR);
    keywords.insert("true".to_string(), TokenType::TRUE);
    keywords.insert("false".to_string(), TokenType::FALSE);
    keywords.insert("null".to_string(), TokenType::NULL);
    keywords.insert("fn".to_string(), TokenType::FN);
    keywords.insert("class".to_string(), TokenType::CLASS);
    keywords.insert("output".to_string(), TokenType::OUTPUT);
    keywords.insert("let".to_string(), TokenType::LET);
    keywords.insert("for".to_string(), TokenType::FOR);
    keywords.insert("if".to_string(), TokenType::IF);
    keywords.insert("else".to_string(), TokenType::ELSE);
    keywords.insert("while".to_string(), TokenType::WHILE);
    keywords.insert("return".to_string(), TokenType::RETURN);
    keywords.insert("super".to_string(), TokenType::SUPER);
}

fn scan_token(lexer_data: &mut LexerData) -> Result<(), shrub_error::SyntaxError> {
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
        '!' => {
            if char_match('=', lexer_data) {
                append_generic_token(TokenType::BANGEQUAL, lexer_data);
            } else {
                append_generic_token(TokenType::BANG, lexer_data);
            }
        }
        '=' => {
            if char_match('=', lexer_data) {
                append_generic_token(TokenType::EQUALEQUAL, lexer_data);
            } else {
                append_generic_token(TokenType::EQUAL, lexer_data);
            }
        }

        '>' => {
            if char_match('=', lexer_data) {
                append_generic_token(TokenType::GREATEREQUAL, lexer_data);
            } else {
                append_generic_token(TokenType::GREATER, lexer_data);
            }
        }

        '<' => {
            if char_match('=', lexer_data) {
                append_generic_token(TokenType::LESSEQUAL, lexer_data);
            } else {
                append_generic_token(TokenType::LESS, lexer_data);
            }
        }

        '/' => {
            if char_match('/', lexer_data) {
                while peek(lexer_data) != '\n' {
                    advance(lexer_data);
                }
            } else {
                append_generic_token(TokenType::SLASH, lexer_data);
            }
        }

        ';' => append_generic_token(TokenType::SEMICOLON, lexer_data),

        ' ' | '\r' | '\t' => (),

        '\n' => lexer_data.line_number += 1,

        '"' => match create_string(lexer_data) {
            Ok(_) => {}
            Err(m) => {
                return Err(SyntaxError {
                    message: m,
                    line_number: lexer_data.line_number,
                })
            }
        },

        _ => {
            if is_number(c) {
                number(lexer_data);
            } else if is_alpha(c) {
                identifier(lexer_data);
            } else {
                return Err(shrub_error::SyntaxError {
                    message: String::from("Unexpected Character"),
                    line_number: lexer_data.line_number,
                });
            }
        }
    }
    Ok(())
}

fn identifier(lexer_data: &mut LexerData) {
    while is_alphanumeric(peek(lexer_data)) {
        if peek(lexer_data) == '\n' {
            lexer_data.line_number += 1
        }
        advance(lexer_data);
    }
    let text: &[char] = &lexer_data.characters[lexer_data.start..lexer_data.current];
    let identifier_name: String = text.iter().collect();

    match lexer_data.key_words.get(&identifier_name) {
        Some(&key_word) => append_generic_token(key_word, lexer_data),
        None => append_generic_token(TokenType::IDENTIFIER, lexer_data),
    }
}

fn number(lexer_data: &mut LexerData) {
    while is_number(peek(lexer_data)) {
        advance(lexer_data);
    }

    if peek(lexer_data) == '.' && is_number(peek_next(lexer_data)) {
        advance(lexer_data);
        while is_number(peek(lexer_data)) {
            advance(lexer_data);
        }
    }

    let text: &[char] = &lexer_data.characters[lexer_data.start..lexer_data.current];
    let string_value: String = text.iter().collect();
    //Add error handling here in the future (if necessary)
    let number_value: f64 = string_value
        .parse::<f64>()
        .expect("Failed to convert to double");
    append_value_token(
        TokenType::NUMBER,
        token::DataType::NUMBER(number_value),
        lexer_data,
    );
}

fn create_string(lexer_data: &mut LexerData) -> Result<(), String> {
    while !is_at_end(lexer_data) && peek(lexer_data) != '"' {
        advance(lexer_data);
    }

    if is_at_end(lexer_data) {
        return Err(String::from("String never terminates."));
    }

    advance(lexer_data);

    let text: &[char] = &lexer_data.characters[lexer_data.start + 1..lexer_data.current - 1];
    let string_value: String = text.iter().collect();
    append_value_token(
        TokenType::STRING,
        token::DataType::TEXT(string_value),
        lexer_data,
    );

    Ok(())
}

fn is_alpha(character: char) -> bool {
    if character >= 'a' && character <= 'z'
        || character >= 'A' && character <= 'Z'
        || character == '_'
    {
        return true;
    }
    return false;
}

fn is_number(character: char) -> bool {
    if character >= '0' && character <= '9' {
        return true;
    }
    return false;
}

fn is_alphanumeric(character: char) -> bool {
    is_alpha(character) || is_number(character)
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

fn append_value_token(token_type: TokenType, value: token::DataType, lexer_data: &mut LexerData) {
    let text: &[char] = &lexer_data.characters[lexer_data.start..lexer_data.current];
    let subtext: String = text.iter().collect();
    let new_token: token::Token = token::Token {
        name: token_type,
        lexeme: subtext,
        value: value,
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

fn peek(lexer_data: &mut LexerData) -> char {
    lexer_data.characters[lexer_data.current]
}

fn peek_next(lexer_data: &mut LexerData) -> char {
    lexer_data.characters[lexer_data.current + 1]
}
