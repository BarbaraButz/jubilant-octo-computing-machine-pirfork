#[derive(Clone, Debug)]
enum Token {
    Number(u32),
    Operator(Op),
    OpeningBracket,
    ClosingBracket,
    //BracketExpression {
    //    opening: char,
    //    token: Box<Token>,
    //    closing: char,
    //},
    //ArithmeticExpression {
    //    first_token: Box<Token>,
    //    oparator: char,
    //    second_token: Box<Token>,
    //},
}

impl Token {
    fn tokenize(string: &mut String) -> Option<Vec<Token>> {
        let mut tokens = Vec::new();
        while !string.is_empty() {
            let last_elem = string.pop();
            match last_elem {
                Some(' ') => {},
                Some('(') => tokens.push(Token::OpeningBracket),
                Some(')') => tokens.push(Token::ClosingBracket),
                Some('+') => tokens.push(Token::Operator(Op::Plus)),
                Some('-') => tokens.push(Token::Operator(Op::Minus)),
                Some('0') => tokens.push(Token::Number(0)),
                Some('1') => tokens.push(Token::Number(1)),
                Some('2') => tokens.push(Token::Number(2)),
                Some('3') => tokens.push(Token::Number(3)),
                Some('4') => tokens.push(Token::Number(4)),
                Some('5') => tokens.push(Token::Number(5)),
                Some('6') => tokens.push(Token::Number(6)),
                Some('7') => tokens.push(Token::Number(7)),
                Some('8') => tokens.push(Token::Number(8)),
                Some('9') => tokens.push(Token::Number(9)),
                _ => return None,
            }
        }
        tokens.reverse();
        return Some(tokens);
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
    Leaf(u32),
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
    fn parse(tokens: &mut [Token]) -> Option<Expr> {
        let length = tokens.len();
        if length == 1 {
            match tokens[0] {
                Token::Number(x) => return Some(Expr::Leaf(x)),
                _ => return None,
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
                            if let Some(expr1) = Expr::parse(child1) {
                                if let Some(expr2) = Expr::parse(child2) {
                                    expression.children_push(expr1);
                                    expression.children_push(expr2);
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
                            if let Some(expr1) = Expr::parse(child1) {
                                if let Some(expr2) = Expr::parse(child2) {
                                    expression.children_push(expr1);
                                    expression.children_push(expr2);
                                }
                            }
                        }
                    },
                    Token::OpeningBracket => bracketcounter += 1,
                    Token::ClosingBracket => bracketcounter -= 1,
                    _ => {},
                }
            }

        }

        return None;
    }

    fn evaluate(&self) -> i32 {
        match *self {
            Expr::Leaf(x) => x as i32,
            Expr::Internal {
                ref operator, ref children
            } =>
            operator.apply(((children[0]).evaluate()), ((children[0]).evaluate()))
        }
    }
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
    println!("{}", expr_test());
    loop {
        // Read input from the user and just do nothing when the input is empty
        let mut input = read_string();
        if input.is_empty() {
            continue;
        }

        let tokenized = Token::tokenize(&mut input);
        match tokenized {
            Some(mut tokens) => {
                // Debug output
                println!("{}", input);
                println!("{:#?}", tokens);
                let parsetree = Expr::parse(&mut tokens);
                if let Some(tree) = parsetree {
                    let result = tree.evaluate();
                    println!("Result: {}", result);
                }
            },
            None => {
                println!("Invalid Input!")
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
