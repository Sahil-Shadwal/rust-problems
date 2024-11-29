enum Shape {
    Circle(f64),
    Rectangle(f64,f64),
}

fn main() {
    let c = Shape::Circle(10.0);
    let r = Shape::Rectangle(10.0, 20.0);

    print_area(c);
    print_area(r);
}

fn print_area(shape: Shape) {
    match shape {
        Shape::Circle(radius) => {
            println!("Circle area: {}", 3.14 * radius * radius);
        },
        Shape::Rectangle(width, height) => {
            println!("Rectangle area: {}", width * height);
        },
    }
}
 
 