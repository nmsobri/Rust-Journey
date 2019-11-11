use std::io;
use std::fs::File;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let file = File::open("users.txt");

    let mut file = match file {
        Ok(file) => file,
        Err(err) => return Err(err)
    };

    let mut string = String::new();

    let result = file.read_to_string(&mut string);

    match result {
        Ok(_) => Ok(string),
        Err(err) => Err(err)
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut file = File::open("users.txt")?;
    let mut string = String::new();

    file.read_to_string(&mut string)?;
    return Ok(string);
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut string = String::new();
    File::open("users.txt")?.read_to_string(&mut string)?;
    return Ok(string);
}

fn main() {
    let user = read_username_from_file3();

    match user {
        Ok(user) => println!("User is: {}", user),
        Err(e) => panic!("Error has occured: {}", e)
    }
}
