#[derive(Clone, Debug)]
enum Token {
    Number(u32),
    Plus,
    Minus,
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
                Some('+') => tokens.push(Token::Plus),
                Some('-') => tokens.push(Token::Minus),
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

fn main() {
    loop {
        // Read input from the user and just do nothing when the input is empty
        let input = read_string();
        if input.is_empty() {
            continue;
        }

        // Debug output
        println!("{}", input);
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
