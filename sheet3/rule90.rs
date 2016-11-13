//! Task 3.1: Rule 90

fn main() {

    let mut vector = read_input();

    for i in 0..20 {
        for j in 0..vector.len() {
            match vector.get(j) {
                Some(&false) => {
                    print!("0");
                },
                Some(&true) => {
                    print!("1");
                },
                _ => {},
            }
        }
        println!("");
        vector = next_step(vector);
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

    let vector = {
        let mut v = Vec::with_capacity(input.chars().count());
        for i in input.chars() {
            match i {
                '0' => {
                    v.push(false);
                },
                '1' => {
                    v.push(true);
                },
                _ => {},

            }
        }
        v
    };
    vector
}

fn next_step(v: Vec<bool>) -> Vec<bool> {
    let mut vector = Vec::new();
    let len = v.len();
    let first_triple = (v.get(len - 1), v.get(0), v.get(1));
    vector.push(calculate(first_triple));
    for i in 1..v.len() - 1 {
        let triple = (v.get(i - 1), v.get(i), v.get(i + 1));
        vector.push(calculate(triple));
    }
    let last_triple = (v.get(len - 2), v.get(len - 1), v.get(0));
    vector.push(calculate(last_triple));
    return vector;
}

fn calculate(neighbours: (Option<&bool>, Option<&bool>, Option<&bool>)) -> bool {
    match neighbours {
        (Some(&false), Some(&false), Some(&false)) => {
            return false;
        },
        //(Some(&false), Some(&false), Some(&true)) => {
        //    return true;
        //},
        (Some(&false), Some(&true), Some(&false)) => {
            return false;
        },
        //(Some(&false), Some(&true), Some(&true)) => {
        //    return true;
        //},
        //(Some(&true), Some(&false), Some(&false)) => {
        //    return true;
        //},
        (Some(&true), Some(&false), Some(&true)) => {
            return false;
        },
        //(Some(&true), Some(&true), Some(&false)) => {
        //    return true;
        //},
        (Some(&true), Some(&true), Some(&true)) => {
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
