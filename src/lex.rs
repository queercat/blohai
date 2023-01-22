use regex::Regex;

#[derive(Debug)]
pub enum Token {
    TokenString {value: String},
    TokenNumber {value: i32},
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
    /// ```no_run
    /// lex("(3 + 4)") -> {TokenLParen, TokenNumber {value = 3}, TokenAdd, TokenNumber {value = 4}, TokenRParen}
    /// ```
    ///

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
            _ => {} 
        }

        if !valid {
            println!("-->{}", text);
            panic!("non-valid token found in token stream");
        }

        return token;
    }

    let mut tokens: Vec<Token> = Vec::new();
    let regex_text = r"[()+-/\\]|\d+|\w+";
    let token_regex = Regex::new(regex_text).unwrap();
    let matches = token_regex.find_iter(source);

    for m in matches {
        tokens.push(create_token(m.as_str()));
    }

    return tokens;
}
