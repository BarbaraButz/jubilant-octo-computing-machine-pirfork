use std::env;
use std::process;

fn main() {
    let filenames = read_filenames();
    if let Err(x) = filenames {
        println!("{:?}", x);
        process::exit(1);
    }
}

#[derive(Debug)]
enum Error {
    InvalidLength,
    NonexistentFile,
}

fn read_filenames() -> Result<Vec<String>, Error> {
    let filenames = env::args().collect::<Vec<String>>();
    //len has to be three because of "target/debug/mycp"-Argument
    match filenames.len() {
        3 => Ok(filenames),
        _ => Err(Error::InvalidLength),
    }
}
