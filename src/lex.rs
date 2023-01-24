use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    TokenNumber {value: i32},
    TokenVariable,
    TokenEquals,
    TokenLParen,
    TokenRParen,
    TokenAdd,
    TokenSubstract,
    TokenMultiply,
    TokenDivide,
    TokenNull,
    EOF,
}

pub fn lex(source: &str) -> Vec<Token> {
    /// Takes a string program as an input and turns it into an array of tokens.
    ///
    /// ```
    /// use blohai::lex;
    /// use lex::Token;
    /// let result = lex::lex("(3 + 4)");
    /// let expected = vec![Token::TokenLParen, Token::TokenNumber {value: 3}, Token::TokenAdd, Token::TokenNumber {value: 4}, Token::TokenRParen];
    /// assert_eq!(result, expected);
    /// ```
    
    fn create_token(text: &str) -> Token {
        let mut token : Token = Token::TokenNull;
        let mut valid = false;

        match text.parse::<i32>() {
            Ok(n) => {valid = true; token = Token::TokenNumber {value: n}}
            Err(_) => {}
        }

        match text {
            "(" => {valid = true; token = Token::TokenLParen}
            ")" => {valid = true; token = Token::TokenRParen}
            "+" => {valid = true; token = Token::TokenAdd}
            "-" => {valid = true; token = Token::TokenSubstract}
            "/" => {valid = true; token = Token::TokenDivide}
            "*" => {valid = true; token = Token::TokenMultiply}
            "=" => {valid = true; token = Token::TokenEquals}
            "var" => {valid = true; token = Token::TokenVariable}
            _ => {} 
        }

        if !valid {
            println!("--> {}", text);
            panic!("non-valid token found in token stream");
        }

        return token;
    }

    let mut tokens: Vec<Token> = Vec::new();
    let regex_text = r"[=()+-/\\]|\d+|\w+";
    let token_regex = Regex::new(regex_text).unwrap();
    let matches = token_regex.find_iter(source);

    for m in matches {
        tokens.push(create_token(m.as_str()));
    }

    return tokens;
}
