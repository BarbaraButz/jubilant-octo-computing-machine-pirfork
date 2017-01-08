use std::collections::HashMap;

macro_rules! hash_map {
    ( $( $key:expr => $val:expr ),* ) => {
        {
            let mut h = HashMap::new();
            $( h.insert($key, $val); )*
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
