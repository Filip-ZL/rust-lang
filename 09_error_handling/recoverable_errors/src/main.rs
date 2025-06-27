use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    
    // if we want to panic straight away we might use unwrap function!
    // let greeting_file = File::open("hello.txt").unwrap();
    // expect:
    // let greeting_file = File::open("Hello.txt").expect("Hello.txt should be included in this project.");
    let greeting_file_result = File::open("Hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file! {e:?}"),
            }
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        }
    };

    // unwrap or else (check after Chapter XIII.)
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut uname_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut uname = String::new();

    match uname_file.read_to_string(&mut uname){
        Ok(_) => Ok(uname),
        Err(e) => Err(e),
    }
}

// same as above --> using ? operator
fn read_uname_from_file() -> Result<String, io::Error> {
    let mut uname_file = File::open("hello.txt")?;
    let uname = String::new();
    uname_file.read_to_string(&mut uname)?;
    Ok(uname)
}

fn read_uname_from_file() -> Result<String, io::Error>{
    let mut uname_file = File::open("hello.txt")

    File::open(uname_file)?.read_to_string(&mut uname);

    Ok(uname)
}

// We can use the fs::read_to_tring