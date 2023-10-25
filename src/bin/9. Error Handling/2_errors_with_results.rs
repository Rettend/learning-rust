use std::error;
use std::fs::{self, File};
use std::io::{self, Error, ErrorKind, Read};

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn main() -> Result<(), Box<dyn error::Error>> {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {error:?}")
        }
    };

    // Matching on Different Errors

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => {
                    panic!("Problem creating the file: {error:?}")
                }
            },
            other_error => {
                panic!("Problem opening the file: {error:?}")
            }
        },
    };

    // Alternative to Using match with Result<T, E>

    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt")
                .unwrap_or_else(|error| panic!("Problem creating the file: {error:?}"))
        } else {
            panic!("Problem opening the file: {error:?}")
        }
    });

    // Shortcuts for Panic on Error: unwrap and expect

    let greeting_file = File::open("hello.txt").unwrap();

    let greeting_file = File::open("hello.txt").expect("Failed to open hello.txt");

    // Propagating Errors

    fn read_username_from_file() -> Result<String, Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    // A Shortcut for Propagating Errors: the ? Operator

    fn read_username_from_file2() -> Result<String, Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }

    fn read_username_from_file3() -> Result<String, Error> {
        let mut username = String::new();

        File::open("hello.txt")?.read_to_string(&mut username)?;

        Ok(username)
    }

    // Or just use fs::read_to_string()

    let username = fs::read_to_string("hello.txt").unwrap();

    // Where the ? operator can be useful

    // let greeting_file = File::open("hello.txt")?; // error!

    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }

    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
