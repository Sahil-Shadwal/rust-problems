struct User {
    first_name: String,
    last_name: String,
    age: i32,
}

fn main(){
    let user = User {
        first_name: String::from("Sahil"),
        last_name: String::from("Shadwal"),
        age:21,
    };
    println!("First Name: {}", user.first_name);
    println!("Last Name: {}", user.last_name);
    println!("Age: {}", user.age);
}