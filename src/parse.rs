use crate::lex;

#[derive(Debug)]
pub enum AST {
    NumberLiteral {value: i32},
    Null,
}

// lookahead
struct TokenStream {
    tokens: Vec<lex::Token>,
    cursor: usize,
}

impl TokenStream {
    // views ahead but doesn't consume.
    fn peek(&mut self) -> &lex::Token {
        return &self.tokens[self.cursor];
    }

    // advances ahead and consumes.
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

            // handle a var.
            lex::Token::TokenVar => {
                let variable = stream.next();
                let operation = stream.peek();

                match operation {
                    lex::Token::TokenSemicolon => {stream.next();}
                    _ => {println!("---> {:?}", operation); panic!("invalid token found while parsing variable declaration.")} 
                }
            }

            // handle a number.
            lex::Token::TokenNumber { value } => {node = AST::NumberLiteral {value: *value}}
            _ => {panic!("invalid token found in token stream")}
        }
    }

    return node;
}
