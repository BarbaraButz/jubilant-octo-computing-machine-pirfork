// print if they are both
fn main() {
    for iterationnumber in 1..20 {
        if happy_prime(iterationnumber) {
            println!("{} is a happy prime!", iterationnumber);
        }
    }

}

// is it both?
fn happy_prime(n: i32) -> bool {
    if check_if_number_is_happy(n) && check_if_number_is_prime(n) {
        return true;
    }
    false
}

// Is it a happy number? https://en.wikipedia.org/wiki/Happy_number
fn check_if_number_is_happy(number: i32) -> bool {
    if number == 1 {
        return true;
    }
    let mut number = number;
    loop {
        let mut help = 0;
        number = {
            while number != 0 {
                help += (number % 10).pow(2);
                number /= 10;
            }
            help
        };

        match number {
            // We ended up in a cycle -> not happy
            4 => return false,
            // We didn't end up in a cycle -> happy :)
            1 => return true,
            _ => {},
        }
    }

}

// is it prime?
fn check_if_number_is_prime(n: i32) -> bool {
    if n == 1 {
        return false;
    }
    for teiler in 2..n {
        if n % teiler == 0 {
            return false;
        }
    }
    true
}
