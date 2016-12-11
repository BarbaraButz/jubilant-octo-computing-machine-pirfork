fn main() {
    print!("Primes: ");
    for n in 0..30 {
        if prime(n) {
            print!("{}, ", n);
        }
    }
    println!("");
    println!("");

    for n in 0..15 {
        println!("{:2}! = {:11}", n, factorial(n));
    }
    println!("");

    let v = vec![1, 2, 0, 4, 100, 2];
    println!("Max of {:?}: {:?}", v.clone(), v.into_iter().max_emulation());
    println!("Max of []: {:?}", (0..0).into_iter().max_emulation())
}

fn prime(n: u64) -> bool {
    match n {
        0 | 1 => false,
        _ => (2..).take_while(|d| d * d <= n).all_emulation(|d| n % d != 0)
    }
}

fn factorial(x: u64) -> u64 {
    (1..x + 1).product_emulation()
}

trait FoldEmulatorExt: Iterator {
    fn all_emulation<F>(&mut self, mut f: F) -> bool
        where F: FnMut(Self::Item) -> bool;

    fn product_emulation(self) -> Self::Item;

    fn max_emulation(self) -> Option<Self::Item> where Self::Item: Ord;
}

impl<T: Iterator<Item = u64>> FoldEmulatorExt for T {
    fn all_emulation<F>(&mut self, mut f: F) -> bool
        where F: FnMut(Self::Item) -> bool
    {
        self.fold(true, |acc, item| acc && f(item))
    }

    fn product_emulation(self) -> Self::Item {
        self.fold(1, |acc, item| acc * item)
    }

    fn max_emulation(self) -> Option<Self::Item> {
        use std::cmp::max;
        self.fold(None, |acc, item| max(acc, Some(item)))
    }
}
