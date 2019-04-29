use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

fn main() -> io::Result<()> {
    let mut args = env::args();
    let _script = args.nth(1).unwrap();
    let filename = args.next().unwrap_or("-".to_string());

    process_filename(&filename)
}

fn process_filename(filename: &str) -> io::Result<()> {
    if filename == "-" {
        process_file(io::stdin())
    } else {
        process_file(File::open(filename)?)
    }
}

fn process_file(file: impl Read) -> io::Result<()> {
    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}
