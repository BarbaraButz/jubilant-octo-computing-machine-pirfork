fn main() {
    for (i, f) in FibIt::new().enumerate().take(20) {
        println!("{:02}: {}", i, f);
    }
}

struct FibIt {
    n_minus_2: u64,
    n_minus_1: u64,
}

impl FibIt {
    fn new() -> Self {
        FibIt {
            n_minus_2: 0,
            n_minus_1: 1,
        }
    }
}

impl Iterator for FibIt {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let n = self.n_minus_1 + self.n_minus_2;
        self.n_minus_2 = self.n_minus_1;
        self.n_minus_1 = n;
        Some(n)
    }
}
