fn main() {

    let mut number = 27;
    let mut counter = 1;

    while number > 1 {
        println!("{} -> {}", counter, number);
        if number % 2 == 0 {
            number = number / 2;
        } else {
            number = 3 * number + 1;
        }
        counter += 1;
    }
    println!("{} -> 1", counter);
}
