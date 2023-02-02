#![allow(dead_code, unused)]
use blohai::lexer;
use lexer::lex;

fn main() {
    let tokens = lex(&"var x = 3");

    println!("{:?}", tokens)
}
