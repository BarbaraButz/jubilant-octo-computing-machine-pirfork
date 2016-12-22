fn main() {
    3.times(|i| {
    println!("Ferris ate {} cookies", i);
});
}

pub trait TimesExt<F> {
    fn times(self, func: F)
    where Self: Sized, F: FnMut(Self);
}

impl<F> TimesExt<F> for u64 {
    fn times(self, mut func: F)
        where F: FnMut(Self) {
        for i in 0..self {
            func(i);
        }
    }
}
