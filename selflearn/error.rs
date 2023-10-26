use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let result = File::open("test.txt");
    let file = match result {
        Ok(file) => file,
        Err(err) => {
            println!("err: {err}");
            match err.kind() {
                ErrorKind::NotFound => match File::create("test.txt") {
                    Ok(file) => file,
                    Err(err) => panic!("Problem creating the file: {:?}", err),
                },
                other_error => panic!("Problem reading the file: {:?}", other_error),
            }
        }
    };
}
