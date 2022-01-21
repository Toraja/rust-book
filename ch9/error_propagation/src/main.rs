use std::fs::{self, File};
use std::io::{self, Read};

fn main() {
    // these do the same thing
    let _ = naive_read_username_from_file();
    let _ = refined_read_username_from_file();
    let _ = more_refined_read_username_from_file();
    let _ = fs::read_to_string("hello.txt");
}

fn naive_read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn refined_read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn more_refined_read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
