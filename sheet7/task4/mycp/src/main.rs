use std::env;
use std::process;
use std::fs::File;
use std::io::BufReader;
use std::io::LineWriter;
use std::io::BufRead;
use std::io::Write;
use std::io::Error;
use std::io::ErrorKind;

fn main() {
    let filenames = read_filenames();
    let files = match filenames {
        Ok(x) => x,
        Err(x) => {
        println!("{:#?}", x);
        process::exit(1);
        }
    };
    if let Err(x) = copy_file(&files[0], &files[1]) {
        println!("{:#?}", x);
        process::exit(1);
    }
}

fn read_filenames() -> Result<Vec<String>, Error> {
    let mut filenames = env::args().collect::<Vec<String>>();
    //len has to be three because of "target/debug/mycp"-Argument
    match filenames.len() {
        3 => {
            let filenames = filenames.split_off(1);
            Ok(filenames)
        }
        _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid length, needs two arguments!")),
    }
}

fn copy_file(oldpath: &String, newpath: &String) -> Result<(), Error> {
    let oldfile = match File::open(oldpath) {
        Ok(x) => x,
        //
        // ändern?
        //
        Err(y) => return Err(y),
    };
    let newfile = match File::create(newpath) {
        Ok(x) => x,
        //
        // ändern?
        //
        Err(x) => return Err(x),
    };
    let reader = BufReader::new(oldfile);
    let mut writer = LineWriter::new(newfile);
    let mut buffer;

    for line in reader.lines() {
        buffer = match line {
            Ok(x) => x,
            Err(x) => return Err(x),
        };
        buffer.push('\n');
        if let Err(x) = writer.write(String::as_bytes(&buffer)){
            return Err(x);
        }
        buffer.clear();
    }
    Ok(())
}
