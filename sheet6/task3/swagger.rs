use std::fmt;

struct Swagger<T> {
    swag: T,
}

impl<T: fmt::Display> fmt::Display for Swagger<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "yolo {} swag", self.swag)
    }
}

trait YoloSwag {
        fn with_swag(&self) -> Swagger<Self>
        where Self: std::marker::Sized + std::clone::Clone;
}

impl<T> YoloSwag for T {
    fn with_swag(&self) -> Swagger<Self>
    where T: std::clone::Clone {
        Swagger{swag: (*self).clone()}
    }
}

fn main() {
    let piep = "piep".to_string();
    let x = 3.with_swag();
    let y = "hallo".with_swag();
    let z = piep.with_swag();
    println!("{}", x);
    println!("{}", y);
    println!("{}", piep);
    println!("{}", z);
}
