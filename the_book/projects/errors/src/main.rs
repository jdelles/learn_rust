use std::fs::{self, File};
use std::io::ErrorKind;

fn main() {
    // a();

    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|e| {
                panic!("Problem creating the file: {:?}", e);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn a() {
    b();
}
fn b() {
    c(22);
}
fn c(num: u32) {
    if num == 22 {
        panic!("Don't pass in 22!");
    }
}

fn read_username_from_file() -> Result<String, std::io::Error> {
    fs::read_to_string("hello.txt")
}
