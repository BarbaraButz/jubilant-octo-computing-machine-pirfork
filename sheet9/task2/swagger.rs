use std::fmt;

struct Swagger<T>(pub T);

macro_rules! swag_fmt_impl {
    ( $( $tr:path => $char:expr, )* ) => {$(
        impl<T: $tr> $tr for Swagger<T> {
            fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                write!(f, "yolo {:$char} swag", self.0)
            }
        }
    )*}
}

impl<T: fmt::Display> fmt::Display for Swagger<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "yolo {} swag", self.0)
    }
}

swag_fmt_impl!{
    fmt::Debug => "?",
    fmt::Binary => "b",
}

trait SwaggerExt: Sized {
    fn with_swag(self) -> Swagger<Self>;
}

impl<T> SwaggerExt for T {
    fn with_swag(self) -> Swagger<Self> {
        Swagger(self)
    }
}

fn main() {
    let pi = 3.14;

    println!("{}", pi);
    println!("{}", Swagger(pi));
    println!("{}", pi.with_swag());

    let arr = vec![1, 2, 3];
    println!("{:?}", arr.with_swag());
}
