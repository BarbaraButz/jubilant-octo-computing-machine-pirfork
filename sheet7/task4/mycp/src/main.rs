use std::env;
use std::process;
use std::fs::File;
use std::io::BufReader;
use std::io::LineWriter;
use std::io::BufRead;
use std::io::Write;
use std::string::String;

fn main() {
    let filenames = read_filenames();
    let files = match filenames {
        Ok(x) => x,
        Err(x) => {
        println!("{:?}", x);
        process::exit(1);
        }
    };
    copy_file(&files[0], &files[1]);
}

#[derive(Debug)]
enum Error {
    InvalidLength,
    NonexistentFile,
}

fn read_filenames() -> Result<Vec<String>, Error> {
    let mut filenames = env::args().collect::<Vec<String>>();
    //len has to be three because of "target/debug/mycp"-Argument
    match filenames.len() {
        3 => {
            let filenames = filenames.split_off(1);
            Ok(filenames)
        }
        _ => Err(Error::InvalidLength),
    }
}

fn copy_file(oldpath: &String, newpath: &String) /*-> Result<(), _> */{
    let oldfile = match File::open(oldpath) {
        Ok(x) => x,
        //
        // ändern!!!!
        //
        Err(_) => process::exit(1),
    };
    let newfile = match File::create(newpath) {
        Ok(x) => x,
        //
        // ändern!!!!
        //
        Err(_) => process::exit(1),
    };
    let mut reader = BufReader::new(oldfile);
    let mut writer = LineWriter::new(newfile);
    let mut buffer = String::new();
    //let bytes = -1;
    while reader.read_line(&mut buffer).unwrap() != 0 {
        //
        // ändern
        //
        let _ = writer.write(String::as_bytes(&buffer));
        buffer.clear();
    }
}
