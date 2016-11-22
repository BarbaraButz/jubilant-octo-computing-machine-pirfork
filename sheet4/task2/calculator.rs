#[derive(Clone, Debug)]
enum Token {
    Number(i32),
    Operator(Op),
    OpeningBracket,
    ClosingBracket,
}

impl Token {
    fn tokenize(string: &mut String) -> Result<Vec<Token>,InputError> {
        let mut tokens = Vec::new();
        let mut number = -1;
        let mut digit = 0;
        while !string.is_empty() {
            let last_elem = string.pop().unwrap();
            match last_elem {
                ' ' => {
                    if number != -1 {
                        tokens.push(Token::Number(number));
                        number = -1;
                        digit = 0;
                    }
                },
                '(' => {
                    if number != -1 {
                        tokens.push(Token::Number(number));
                        number = -1;
                        digit = 0;
                    }
                    tokens.push(Token::OpeningBracket);
                }
                ')' => {
                    if number != -1 {
                        tokens.push(Token::Number(number));
                        number = -1;
                        digit = 0;
                    }
                    tokens.push(Token::ClosingBracket);
                }

                '+' => {
                    if number != -1 {
                        tokens.push(Token::Number(number));
                        number = -1;
                        digit = 0;
                    }
                    tokens.push(Token::Operator(Op::Plus));
                }
                '-' => {
                    if number != -1 {
                        tokens.push(Token::Number(number));
                        number = -1;
                        digit = 0;
                    }
                    tokens.push(Token::Operator(Op::Minus));
                }
                '0' => {
                    if number == -1 {
                        number = 0;
                    }
                    digit += 1;
                },
                '1' => {
                    if number == -1 {
                        number = 1;
                    } else {
                        number += 10_i32.pow(digit);
                    }
                    digit += 1;

                },
                '2' => {
                    if number == -1 {
                        number = 2;
                    } else {
                        number += 2 * 10_i32.pow(digit);
                    }
                    digit += 1;
                },
                '3' => {
                    if number == -1 {
                        number = 3;
                    } else {
                        number += 3 * 10_i32.pow(digit);
                    }
                    digit += 1;
                },
                '4' => {
                    if number == -1 {
                        number = 4;
                    } else {
                        number += 4 * 10_i32.pow(digit);
                    }
                    digit += 1;
                },
                '5' => {
                    if number == -1 {
                        number = 5;
                    } else {
                        number += 5 * 10_i32.pow(digit);
                    }
                    digit += 1;
                },
                '6' => {
                    if number == -1 {
                        number = 6;
                    } else {
                        number += 6 * 10_i32.pow(digit);
                    }
                    digit += 1;
                },
                '7' => {
                    if number == -1 {
                        number = 7;
                    } else {
                        number += 7 * 10_i32.pow(digit);
                    }
                    digit += 1;
                },
                '8' => {
                    if number == -1 {
                        number = 8;
                    } else {
                        number += 8 * 10_i32.pow(digit);
                    }
                    digit += 1;
                },
                '9' => {
                    if number == -1 {
                        number = 9;
                    } else {
                        number += 9 * 10_i32.pow(digit);
                    }
                    digit += 1;
                },
                x => return Err(InputError::TokenizationError{invalid: x}),
            }
        }
        if number != -1 {
            tokens.push(Token::Number(number));
        }
        tokens.reverse();
        Ok(tokens)
    }
}

#[derive(Clone, Debug)]
enum Op {
    Plus,
    Minus,
}

impl Op {
    fn apply(&self, number1: i32, number2: i32) -> i32 {
        match *self {
            Op::Plus => {
                number1 + number2
            },
            Op::Minus => {
                number1 - number2
            },
        }
    }
}

#[derive(Clone, Debug)]
enum Expr {
    Leaf(i32),
    Internal {
        operator: Op,
        children: Vec<Expr>,
    },
}

impl Expr {
    fn children_push(&mut self, expression: Expr) {
        if let Expr::Internal{ref mut children, ..} = *self {
            children.push(expression);
        }
    }
    fn parse(tokens: &mut [Token]) -> Result<Expr, InputError> {
        let length = tokens.len();
        if length == 0 {
            return Err(InputError::ParseEmptyError);
        }
        if length == 1 {
            match tokens[0] {
                Token::Number(x) => return Ok(Expr::Leaf(x)),
                _ => return Err(InputError::ParseInvalidError),
            }
        }
        let mut bracketcounter = 0;
        let mut expression;
        for i in 0..length {
            if bracketcounter == 0 {
                match tokens[i] {
                    Token::Operator(Op::Plus) => {
                        expression = Expr::Internal {
                            operator: Op::Plus,
                            children: vec![],
                        };
                        let (mut child1, mut rest) = tokens.split_at_mut(i);
                        if let Some((_, mut child2)) = rest.split_first_mut() {
                            if let Ok(expr1) = Expr::parse(child1) {
                                if let Ok(expr2) = Expr::parse(child2) {
                                    expression.children_push(expr1);
                                    expression.children_push(expr2);
                                    return Ok(expression);
                                }
                            }
                        }
                    },
                    Token::Operator(Op::Minus) => {
                        expression = Expr::Internal {
                            operator: Op::Minus,
                            children: vec![],
                        };
                        let (mut child1, mut rest) = tokens.split_at_mut(i);
                        if let Some((_, mut child2)) = rest.split_first_mut() {
                            if let Ok(expr1) = Expr::parse(child1) {
                                if let Ok(expr2) = Expr::parse(child2) {
                                    expression.children_push(expr1);
                                    expression.children_push(expr2);
                                    return Ok(expression);
                                }
                            }
                        }
                    },
                    _ => {},
                }
            } else {
                match tokens[i] {
                    Token::OpeningBracket => bracketcounter += 1,
                    Token::ClosingBracket => bracketcounter -= 1,
                    _ => {},
                }
            }
        }
        if let Token::OpeningBracket = tokens[0] {
            if let Token::ClosingBracket = tokens[length-1] {
                if let Some((_, elements)) = tokens.split_first_mut() {
                    if let Some((_, newelements)) = elements.split_last_mut() {
                        return Expr::parse(newelements);
                    }
                }
            }
        }
        Err(InputError::ParseInvalidError)
    }

    fn evaluate(&self) -> i32 {
        match *self {
            Expr::Leaf(x) => x as i32,
            Expr::Internal {
                ref operator, ref children
            } => operator.apply(((children[0]).evaluate()), ((children[1]).evaluate()))
        }
    }
}

enum InputError {
    TokenizationError {
        invalid: char
    },
    ParseEmptyError,
    ParseInvalidError,

}

fn expr_test() -> bool {
    let leaf1 = Expr::Leaf(3);
    let leaf2 = Expr::Leaf(6);
    let leaf3 = Expr::Leaf(1);
    let internal1 = Expr::Internal{operator: Op::Minus, children: vec![leaf2, leaf3]};
    let internal2 = Expr::Internal{operator: Op::Plus, children: vec![leaf1, internal1]};
    internal2.evaluate() == 8
}

fn main() {
    println!("evaluate()-test passed? {}", expr_test());
    loop {
        // Read input from the user and just do nothing when the input is empty
        let mut input = read_string();
        if input.is_empty() {
            continue;
        }

        let tokenized = Token::tokenize(&mut input);
        match tokenized {
            Ok(mut tokens) => {
                let parsetree = Expr::parse(&mut tokens);
                match parsetree {
                    Ok(tree) => {
                        let result = tree.evaluate();
                        println!("Result: {}", result);
                    },
                    Err(InputError::ParseEmptyError) => {
                        println!("Nothing to calculate!");
                    },
                    Err(InputError::ParseInvalidError) => {
                        println!("Invalid Order!");
                    },
                    _ => println!("Something very strange happened here..."),
                }
            },
            Err(error) => {
                if let InputError::TokenizationError{invalid} = error {
                    println!("Invalid Character! -> {}", invalid);
                } else {
                    println!("Something very strange happened here...");
                }

            },
        }


    }
}


/// Reads a string from the user (with a nice prompt).
fn read_string() -> String {
    use std::io::Write;

    // Print prompt
    print!("calc > ");
    std::io::stdout().flush().unwrap();

    // Read line
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("something went horribly wrong...");

    // Discard trailing newline
    let new_len = buffer.trim_right().len();
    buffer.truncate(new_len);

    buffer
}
