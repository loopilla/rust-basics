// Polymorphism with enums

enum Shape {
    Rectangle {     // Struct
        width: u32,
        height: u32
    },
    Square(u32),    // Tuple
    Circle(f64)
}


impl Shape {
    fn area(&self) -> f64 { //Sould have the same output f64
        match *self {
            Shape::Rectangle {width, height} => (width * height) as f64,
            Shape::Square(ref s) => (s * s) as f64,
            Shape::Circle(ref r) => 3.1415 * (r * r)
        }
    }
}

fn main() {
    let rect = Shape::Rectangle {
        width: 10,
        height: 5
    };
    let square = Shape::Square(5);
    let circle = Shape::Circle(2.0f64);

    println!("{}", rect.area());
    println!("{}", square.area());
    println!("{}", circle.area());
}
