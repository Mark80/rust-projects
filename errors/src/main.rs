use std::fs::File;
use std::io::{Error, ErrorKind, Read};

fn main() {
    let file = File::open("hello.txt");

    let result = match file {
        Ok(f) => f,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) =>
                    fc,
                Err(e) => {
                    panic!("Problem creating the file: {:?}", e)
                }
            }
            _ =>
                panic!("Unexpected error")
        }
    };

    file.unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem creating the file: {:?}", error)
        }
    });

    match read_file("hello.txt"){
        Ok(s) => println!("The content is {}", s),
        Err(error) =>  panic!("Problem creating the file: {:?}", error)
    }
}

fn read_file(path: &str) -> Result<String, Error> {
    let file = File::open("hello.txt");

    let mut file = match file {
        Ok(file) => file,
        Err(err) => return Err(err)
    };

    let mut name = String::new();

    match file.read_to_string(&mut name) {
        Ok(s) => Ok(name),
        Err(err) => Err(err)
    }

}

fn read_file_v2(path: &str) ->  Result<String, Error> {
    let mut name = String::new();

    File::open("hello.txt")?.read_to_string(&mut name)?;
    Ok(name)

}
