use std::{env, process};
use std::fs::File;
use std::io::{BufReader, LineWriter, BufRead, Write};
use std::io::{Error, ErrorKind};

fn main() {
    let filenames = match read_filenames() {
        Ok(x) => x,
        Err(x) => {
            println!("{}", x);
            process::exit(1);
        }
    };
    if let Err(x) = copy_file(&filenames[0], &filenames[1]) {
        println!("{}", x);
        process::exit(1);
    }
}

fn read_filenames() -> Result<Vec<String>, Error> {
    let mut filenames = env::args().collect::<Vec<String>>();
    //len has to be three because of "target/debug/mycp"-Argument
    match filenames.len() {
        3 => Ok(filenames.split_off(1)),
        _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid length, needs two arguments!")),
    }
}

fn copy_file(old_path: &str, new_path: &str) -> Result<(), Error> {
    let old_file = try!(File::open(old_path));
    let new_file = try!(File::create(new_path));

    let reader = BufReader::new(old_file);
    let mut writer = LineWriter::new(new_file);

    for line in reader.lines() {
        let mut buffer = try!(line);
        buffer.push('\n');
        try!(writer.write(String::as_bytes(&buffer)));
    }
    Ok(())
}
