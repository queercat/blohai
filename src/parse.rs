use crate::lex;

pub fn parse(source: &str) {
    lex::lex(source);
}
