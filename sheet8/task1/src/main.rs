fn main() {
    3.times(|i| {
        println!("Ferris ate {} cookies", i);
    });
}

pub trait TimesExt {
    fn times<F>(self, func: F)
    where Self: Sized, F: FnMut(Self);
}

impl TimesExt for u64 {
    fn times<F>(self, mut func: F)
    where F: FnMut(Self) {
        for i in 0..self {
            func(i);
        }
    }
}
