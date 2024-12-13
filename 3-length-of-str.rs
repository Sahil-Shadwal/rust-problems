// fn main() {
//     let name = &String::from("Shadwal");
//     let len = get_str_len(name);
//     println!("Hello, {}!", name);
//     println!("The length of your name is, {}!", len);

// }

// fn get_str_len(s: &String) -> usize {
//     s.chars().count()
// }

fn main(){
    let name = String::from("Shadwal");
    let len = get_str_len(name);
    println!("Hello, {}!", len);
}

fn get_str_len(str: String) -> usize {
    str.chars().count()
}