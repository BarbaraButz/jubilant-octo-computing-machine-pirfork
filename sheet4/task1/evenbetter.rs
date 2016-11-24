// print if they are both
fn main() {
    for n in 1..20 {
        if happy_prime(n) {
            println!("{} is a happy prime!", n);
        }
    }

}

// is it both?
fn happy_prime(n: i32) -> bool {
    happy_check(n) && prime_check(n)
}

// Is it a happy number? https://en.wikipedia.org/wiki/Happy_number
fn happy_check(mut n: i32) -> bool {
    if n == 1 {
        return true;
    }
    while n != 1 {
        let mut help = 0;
        n = {
            while n != 0 {
                help += (n % 10).pow(2);
                n /= 10;
            }
            help
        };
        // We ended up in a cycle -> not happy
        if n == 4 {
            return false;
        }
    }
    true
}

// is it prime?
fn prime_check(n: i32) -> bool {
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
