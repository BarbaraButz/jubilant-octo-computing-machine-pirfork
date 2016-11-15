//! Task 3.1: Rule 90

fn main() {

    let mut vector = read_input();

    for _ in 0..20 {
        for elem in &vector {
            match elem {
                &false => {
                    print!("0");
                },
                &true => {
                    print!("1");
                },
            }
        }
        println!("");
        vector = next_step(&vector);
    }
}

/// Reads a valid initial configuration for our automaton from the terminal.
fn read_input() -> Vec<bool> {
    // This tries to read a string from the terminal, checks whether it's
    // valid (only contains 1's and 0's). If the user fails to input a correct
    // string, this routine will ask again until the user finally manages to
    // give us a correct string.
    //
    // You don't need to understand this routine yet; that's why I've written
    // it already ;-)
    //
    // You only need to use the `input` variable (of type `String`). You can
    // also assume that it only contains '0' and '1' chars.
    let input = {
        let mut buffer = String::new();

        loop {
            println!("Please give me the initial configuration (a string of '0' and '1'!):");
            buffer.clear();

            // `read_line` returns an error if the input isn't valid UTF8 or if
            // a strange IO error occured. We just panic in that case...
            std::io::stdin()
                .read_line(&mut buffer)
                .expect("something went seriously wrong :O");

            if buffer.trim().chars().all(|c| c == '1' || c == '0') {
                break;
            }
        }

        buffer.trim().to_string()
    };

    let mut vector = Vec::new();
    for i in input.chars() {
        match i {
            '0' => {
                vector.push(false);
            },
            '1' => {
                vector.push(true);
            },
            _ => {},

        }
    }
    vector
}

fn next_step(v: &[bool]) -> Vec<bool> {
    let mut vector = Vec::new();
    let len = v.len();
    let first_triple = (v[len - 1], v[0], v[1]);
    vector.push(calculate(first_triple));
    for i in 1..v.len() - 1 {
        let triple = (v[i - 1], v[i], v[i + 1]);
        vector.push(calculate(triple));
    }
    let last_triple = (v[len - 2], v[len - 1], v[0]);
    vector.push(calculate(last_triple));
    vector
}

fn calculate(neighbours: (bool, bool, bool)) -> bool {
    match neighbours {
        (false, false, false) => {
            return false;
        },
        (false, true, false) => {
            return false;
        },
        (true, false, true) => {
            return false;
        },
        (true, true, true) => {
            return false;
        },
        (_, _, _) => {},
    };
    true
}

#[test]
fn rule90_rules() {
    assert_eq!(next_step(&[false, false, false]), vec![false, false, false]);
    assert_eq!(next_step(&[ true, false, false]), vec![false,  true,  true]);
    assert_eq!(next_step(&[ true,  true, false]), vec![ true,  true, false]);
    assert_eq!(next_step(&[ true,  true,  true]), vec![false, false, false]);
}
