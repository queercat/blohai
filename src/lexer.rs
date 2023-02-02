use regex::Regex;

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Eq
}

#[derive(Debug)]
pub enum Token {
    Definition,
    Identifier {value: String},
    Number {value: f32},
    Operator {value: BinaryOperator},
    Null
}

pub fn lex (source: &str) -> Vec<Token> {
    let mut tokens : Vec<Token> = vec![];

    let language_set : Vec<&str> = 
        vec![
            r"[A-Za-z]+",// valid variable name 0
            r"-?\d+\.?\d+|-?\d+", // numbers 1 
            r"[/\*\+-=]$", // binary operator 2
            r"var", // variable declaration 3
        ];

    let words: Vec<&str> = source.split_whitespace().collect();

    // this could be optimized.
    // create a vector of look ups for regexs after iterating over the language_set
    // when referencing a regex rule just access from the index
    // allocation inside for loop is not ideal.

    for word in words {
        let mut token : Token = Token::Null; 

        let mut matched = false;

        for (i, res) in language_set.iter().enumerate() {
            let re = Regex::new(res).unwrap();

            if (re.is_match(word) && !matched) {
                match i {
                    0 => {token = Token::Identifier {value: String::from(word)}}
                    1 => {token = Token::Number {value: word.parse::<f32>().unwrap()}}
                    2 => {
                        let mut operator : BinaryOperator;

                        match word {
                            "+" => {operator = BinaryOperator::Add}
                            "-" => {operator = BinaryOperator::Sub}
                            "/" => {operator = BinaryOperator::Div}
                            "*" => {operator = BinaryOperator::Mul}
                            "=" => {operator = BinaryOperator::Eq}
                            _ => {panic!("failure to match binary operator to known rule... this shouldn't happen, input was {}", word)}
                        }

                        token = Token::Operator {value: operator}
                    }
                    3 => {token = Token::Definition}
                    _ => {panic!("non-valid input --> {} <-- found while reading input source", word)}
                }

                matched = true;
            }
        }

        tokens.push(token);
    }

    tokens
}
