

struct Swagger<T>(pub T);

macro_rules! swag_fmt_string {
    (Display) => ("yolo {} swag");
    (Debug) => ("yolo {:?} swag");
    (Binary) => ("yolo {:b} swag");
    (Octal) => ("yolo {:o} swag");
    (LowerHex) => ("yolo {:x} swag");
    (UpperHex) => ("yolo {:X} swag");
    (LowerExp) => ("yolo {:e} swag");
    (UpperExp) => ("yolo {:E} swag");
}

macro_rules! swag_fmt_impl {
    ( $( $trait_:ident ),* ) => (
        use std::fmt;
        $(
            impl<T: fmt::$trait_> fmt::$trait_ for Swagger<T> {
                fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                    write!(f, swag_fmt_string!($trait_), self.0)
                }
            }
        )*
    );
}



swag_fmt_impl!(Debug, Display, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp);

trait SwaggerExt: Sized {
    fn with_swag(self) -> Swagger<Self>;
}

impl<T> SwaggerExt for T {
    fn with_swag(self) -> Swagger<Self> {
        Swagger(self)
    }
}

fn main() {
    let pi = 3.14.with_swag();

    println!("{}", pi);
    println!("{:e}", pi);
    println!("{:E}", pi);

    let arr = vec![1, 2, 3];
    println!("{:?}", arr.with_swag());

    let answer = 42.with_swag();
    println!("{:x}", answer);
    println!("{:X}", answer);
    println!("{:b}", answer);
    println!("{:o}", answer);


}
