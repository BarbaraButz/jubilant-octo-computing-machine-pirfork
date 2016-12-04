use std::fmt;

struct Swagger<T> {
    yolo: T,
}

impl<T> Swagger<T> {

}

impl<T: fmt::Display> fmt::Display for Swagger<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "yolo {} swag", self.yolo)
    }
}

trait SwaggerExt {
        fn with_swag(self) -> Swagger<Self>
        where Self: std::marker::Sized;
}

impl<T> SwaggerExt for T {
    fn with_swag(self) -> Swagger<Self> {
        Swagger{yolo: self}
    }
}

fn main() {
    let x = 3.with_swag();
    let y = "hallo".with_swag();
    println!("{}", x);
    println!("{}", y);
}
