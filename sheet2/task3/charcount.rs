fn main() {
    let teststring = "Rust ist toll!".to_string();

    println!("{}", count(&teststring, 'R'));
    println!("{}", count(&teststring, 'u'));
    println!("{}", count(&teststring, 's'));
    println!("{}", count(&teststring, 't'));
}




fn count(string: &String, character: char) -> i32 {
    let mut counter = 0;
    for c in string.chars(){
        if c == character {
            counter += 1;
        }
    }
    counter
}
