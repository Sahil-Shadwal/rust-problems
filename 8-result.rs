use std::fs::read_to_string;

fn main(){
    let result = read_to_string("a.text");

    match result {
        Ok(data) => println!("{}", data),
        Err(error) => println!("Error while reading the file"),
    }
}