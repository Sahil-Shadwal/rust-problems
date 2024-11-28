struct Rect {
    width : i32,
    height : i32,
}

impl Rect {
    fn area(&self) -> i32 {
        self.width * self.height
    }

}

fn main() {
    let rect = Rect {
        width: 10,
        height: 20,
    };

    println!("Area is {}", rect.area());
}