use crate::lex;

#[derive(Debug)]
pub enum AST {
    NumberLiteral {value: i32},
    Null,
}

struct TokenStream {
    tokens: Vec<lex::Token>,
    cursor: usize,
}

impl TokenStream{
    fn peek(&mut self) -> &lex::Token {
        return &self.tokens[self.cursor];
    }

    fn next(&mut self) -> &lex::Token {
        self.cursor += 1;
        return &self.tokens[self.cursor - 1];
    }

}

///https://stackoverflow.com/questions/21747136/how-do-i-print-in-rust-the-type-of-a-variable
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

pub fn parse(source: &str) -> AST {
    let tokens = lex::lex(source); 
    let mut stream = TokenStream {tokens: tokens, cursor: 0};

    let mut node : AST = AST::Null;

    while stream.cursor < stream.tokens.len() {
        let token = stream.next();

        match token {
            lex::Token::TokenNumber { value } => {node = AST::NumberLiteral {value: *value}}
            _ => {panic!("invalid token found in token stream")}
        }
    }

    return node;
}
