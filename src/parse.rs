use crate::lex;

pub struct TokenStream {
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

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub enum BinaryOperand {
    Variable {index: u32, name: String}
}

#[derive(Debug)]
pub enum Statement {
    DeclareVariable {index: u32, name: String, value: i32},
    BinaryOperation {operator: BinaryOperator, lhs: BinaryOperand, rhs: BinaryOperand}
}

/// Given some expression input returns an expression statement.
///
/// (3 + 4) => BinaryOperation {operator: BinarySum, lhs: Number, rhs: Number}
///
/// ((x + y) - (3 / 2)) => BinaryOperation {operator: BinarySubtract, lhs: BinaryOperation
/// {operator: BinarySum, lhs: Variable{x} rhs: Variable {y}}, rhs: Binary Operation {operator:
/// BinaryDivide, lhs: Number {3}, rhs: Number {4}}
///
///  ```
///  use blohai::parse;
/// 
///  ```
pub fn evaluate_expression(stream: &TokenStream) {

    

}

pub fn parse(source: &str) -> Vec<Statement> {
    let tokens = lex::lex(source); 

    let mut statements: Vec<Statement> = vec![];
    let mut stream = TokenStream {tokens: tokens, cursor: 0};

    while stream.cursor < stream.tokens.len() {
        let token = stream.next();

        match token {
            lex::Token::TokenVar => {
                // TokenStream -> {var, {variable}, operator ...
                let variable = stream.next();
                let mut statement = Statement::DeclareVariable {index: 0, name: String::from(""), value: -1};

                let mut v_name : String;
                let mut v_index : u32 = 0;

                match variable {
                    lex::Token::TokenVariable {name, index} => {v_name = name.to_string(); v_index = *index;},
                    _ => {panic!("expected Token variable in Token stream but found something else.")}
                }

                let operator = stream.peek();
                match operator {
                    // handle variable declaration `var x;`.
                    lex::Token::TokenSemicolon => {stream.next(); statement = Statement::DeclareVariable {index: v_index, name: v_name, value: 0};},
                    
                    _ => {panic!("unsupported operation found in variable declaration")}
                }

                statements.push(statement);
            }
            _ => {panic!("non-valid token found in token stream")}
        }
    }

    for statement in &statements {
        println!("{:?}", statement);
    }

    return statements;
}
