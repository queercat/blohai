use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    TokenNumber {value: i32},
    // keyword
    TokenVar,
    // literal actual variable
    TokenVariable {name: String, index: u32},
    TokenEquals,
    TokenLParen,
    TokenRParen,
    TokenAdd,
    TokenSubstract,
    TokenMultiply,
    TokenDivide,
    TokenNull,
    TokenSemicolon,
    EOF,
}

/// Creates a symbol table by finding all the variable declarations and indexing them.
///
/// ```
/// use blohai::lex;
/// let result = lex::create_symbol_table("var x; var y;");
///
/// assert_eq!(result["x"], 0);
/// assert_eq!(result["y"], 1);
/// ```
pub fn create_symbol_table(source: &str) -> HashMap<&str, u32> {
    let mut symbol_table = HashMap::new();
    let mut local_symbol_index = 0;

    let symbol_regex_text = r"var \w+";
    let symbol_regex = Regex::new(symbol_regex_text).unwrap();
    let matches = symbol_regex.find_iter(source); 

    // guaranteed at least 2 per match due to regex.
    for m in matches {
        let splits: Vec<&str> = m.as_str().split(" ").collect();
        let symbol = splits[1];

        if symbol_table.contains_key(symbol) {
            panic!("redclaration of previously used symbol {} found", symbol);
        }

        symbol_table.insert(symbol, local_symbol_index);
        local_symbol_index += 1;
    }

    return symbol_table;
}

/// Creates and returns the proper Token enum given some input and a symbol table.
///
/// ```
/// use blohai::lex;
/// let text = vec!["32", "("];
/// let symbol_table = lex::create_symbol_table("");
///
/// let number_token = lex::create_token(text[0], &symbol_table);
/// let paren_token = lex::create_token(text[1], &symbol_table);
///
/// assert_eq!(number_token, lex::Token::TokenNumber {value: 32});
/// assert_eq!(paren_token, lex::Token::TokenLParen);
/// ```
///
pub fn create_token(text: &str, symbol_table: &HashMap<&str, u32>) -> Token {
    let mut token : Token = Token::TokenNull;
    let mut valid = false;

    match text.parse::<i32>() {
        Ok(n) => {valid = true; token = Token::TokenNumber {value: n}}
        Err(_) => {}
    }

    if !valid {
        match text {
            "(" => {valid = true; token = Token::TokenLParen}
            ")" => {valid = true; token = Token::TokenRParen}
            "+" => {valid = true; token = Token::TokenAdd}
            "-" => {valid = true; token = Token::TokenSubstract}
            "/" => {valid = true; token = Token::TokenDivide}
            "*" => {valid = true; token = Token::TokenMultiply}
            "=" => {valid = true; token = Token::TokenEquals}
            ";" => {valid = true; token = Token::TokenSemicolon}
            "var" => {valid = true; token = Token::TokenVar}
            // check if this is a symbol we know.
            _ => {
                if !symbol_table.contains_key(text) {
                    panic!("unknown symbol found in input stream");
                } 
               
                valid = true;
                token = Token::TokenVariable {name: String::from(text), index: symbol_table[text]};
            }
        }
    }

    if !valid {
        println!("--> {}", text);
        panic!("non-valid token passed in token stream");
    }

    return token;
}

/// Takes a string program as an input and turns it into an array of tokens.
///
/// ```
/// use blohai::lex;
/// use lex::Token;
/// let result = lex::lex("(3 + 4)");
/// let expected = vec![Token::TokenLParen, Token::TokenNumber {value: 3}, Token::TokenAdd, Token::TokenNumber {value: 4}, Token::TokenRParen];
/// assert_eq!(result, expected);
/// ```
pub fn lex(source: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let regex_text = r"[=()+-/\\;]|\d+|\w+";
    let token_regex = Regex::new(regex_text).unwrap();
    let matches = token_regex.find_iter(source);
    let symbol_table = create_symbol_table(source);

    for m in matches {
        tokens.push(create_token(m.as_str(), &symbol_table));
    }

    return tokens;
}
