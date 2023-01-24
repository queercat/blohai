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

pub fn parse(source: &str) -> AST {
    let tokens = lex::lex(source); 
    
    let mut stream = TokenStream {tokens: tokens, cursor: 0};
    let mut node : AST = AST::Null;

    while stream.cursor < stream.tokens.len() {
        let token = stream.next();

        println!("{:?}", token);

        match token {
            // start expression.
            lex::Token::TokenLParen => {
            }

            // handle variable definition.
            lex::Token::TokenVariable => {
                 
            }

            // handle a number.
            lex::Token::TokenNumber { value } => {node = AST::NumberLiteral {value: *value}}
            _ => {panic!("invalid token found in token stream")}
        }
    }

    return node;
}
