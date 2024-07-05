use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

// this function itself, handling the error
pub fn opening_file() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {}", e),
            },
            other_error => {
                panic!("Problem opening the file: {}", other_error);
            }
        },
    };
}
// this function will return the error to the calling code, calling code will handle the error
pub fn read_data_from_file() -> Result<String, io::Error> {
    //  let data_file_result=File::open("Hello.txt");
    //  let mut data_result=match data_file_result {
    //      Ok(file)=>file,
    //      Err(e)=>return Err(e),
    //  };
    //  let mut username =String::new();
    //  match data_result.read_to_string(&mut username) {
    //      Ok(_)=>Ok(username),
    //      Err(e)=>Err(e),
    //  }

    // using ? operator
    // let mut read_data = File::open("Hello.txt")?;
    // let mut username = String::new();
    // read_data.read_to_string(&mut username)?;
    // Ok(username)

    //  shorter
    // let mut data=String::new();
    // File::open("hello.txt")?.read_to_string(&mut data)?;
    // Ok(data)

    // shortest
    fs::read_to_string("hello.txt")
}

// function to show the behaviour of ? using on Option enum
pub fn option_enum(text:&str)->Option<char>{
  text.lines().next()?.chars().last()
}

// Note:- Point to remember 
  // The ? operator can only be used in functions whose return type us compatible with the value the ?
  // is used on.