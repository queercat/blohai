mod lex;
mod parse;

fn main() {
    parse::parse("(42 + 3)")
}

