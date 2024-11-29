enum CustomOptio {
    Some(i32),
    None,
}

fn main() {
    let index = find_first_a(String::from("hello"));

    match index {
        CustomOptio::Some(i) => println!("Found first a at index: {}", i),
        CustomOptio::None => println!("No a's found in the string"),
    }
}

fn find_first_a(s: String) -> CustomOptio {
    for (i, c) in s.chars().enumerate() {
        if c == 'a' {
            return CustomOptio::Some(i as i32);
        }
    }

    CustomOptio::None
}