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

//this in java
/*
class Rect {
    int width;
    int height;

    Rect(int width, int height) {
        this.width = width;
        this.height = height;
    }

    int area() {
        return this.width * this.height;
    }
}
    System.out.println(new Rect(10, 20).area());
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