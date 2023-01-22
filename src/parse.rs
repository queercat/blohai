use crate::lex;

struct TokenStream {
    tokens: Vec<lex::Token>,
    cursor: usize,
}

impl TokenStream {
    fn next(&mut self) -> &lex::Token {
        self.cursor += 1;
        return &self.tokens[self.cursor - 1];
    }

    fn peek(&mut self) -> &lex::Token {
        return &self.tokens[self.cursor + 1];
    }
}

pub fn parse(source: &str) {
    let tokens = lex::lex(source); 
    
    let mut stream = TokenStream {tokens: tokens, cursor: 0};

    while stream.cursor < stream.tokens.len() - 1 {
        let token = stream.next();

        match token {
            _ => {panic!("non-valid token found in token stream")}
        }
    }
}
