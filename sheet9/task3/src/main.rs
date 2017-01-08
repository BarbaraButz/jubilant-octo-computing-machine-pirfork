use std::collections::HashMap;

macro_rules! hash_map {
    ( $( $x:expr => $y:expr ),* ) => {
        {
            let mut h = HashMap::new();
            $( h.insert($x, $y); )*
            h
        }
    };
}

fn main() {
    println!("{}", "Mit Makro: ");
    let ages = hash_map!{ "Sabine" => 26, "Peter" => 32 };
    println!("{:#?}", ages);
    println!("{}", "Ohne Makro: ");
    let mut test = HashMap::new();
    test.insert("Sabine", 26);
    test.insert("Peter", 32);
    println!("{:#?}", test);
}
