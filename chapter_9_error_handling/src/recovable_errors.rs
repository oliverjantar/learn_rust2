use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};

pub fn run() {
    open_file();
    open_file_again();
}

fn open_file() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
            other_error => panic!("Problem opening file: {:?}", other_error),
        },
    };
}

fn open_file_again() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating file {:?}", error);
            })
        } else {
            panic!("problem opening file {:?}", error);
        }
    });
}

fn other_handling() {
    let f = File::open("hello.txt").unwrap();
    /*
        returns file if exists or calls panic. Similar to:

    match f {
        Ok(file)=> file,
        Err(e)=>panic!()
    }

    */

    let f = File::open("hello.txt").expect("Some reasonable error message");
}

fn read_username_from_file(path: &str) -> Result<String, io::Error> {
    let f = File::open(path);

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

/* cannot return err from closure
fn read_username_from_file_2(path: &str) -> Result<String, io::Error> {
    let f = File::open(path).unwrap_or_else(|e| return Err(e));

    let mut s = String::new();

    f.read_to_string(&mut s).unwrap_or_else(|e| return Err(e));

    Ok(s)
}
*/

fn read_username_from_file_3(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;

    let mut s = String::new();

    f.read_to_string(&mut s)?;

    Ok(s)
}

fn even_shorter(path: &str) -> Result<String, io::Error> {
    let mut s = String::new();

    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}

fn the_shortest(path: &str) -> Result<String, io::Error> {
    std::fs::read_to_string(path)
}
