struct Rect {
    width : i32,
    height : i32,
}
// this in a js class
/*
class Rect {
    constructor(width, height) {
        this.width = width;
        this.height = height;
    }

    area() {
        return this.width * this.height;
    }
}
    const rect = new Rect(10, 20);
    console.log(rect.area());
*/
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