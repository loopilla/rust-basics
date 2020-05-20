use std::fmt;

#[derive(Debug)]
struct Object {
    width: u32,
    height: u32
}


impl Object {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn new(width: u32, height: u32) -> Object {
        Object {
            width,
            height
        }
    }

    fn wider(&self) -> Object {
        Object::new(self.width + 1, self.height)
    }

    fn add(&mut self) -> &mut Self {
        self.height += 1;
        self
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.width, self.height)
    }
}

fn main() {
    let mut o = Object::new(12, 18);

    println!("{}", o);
    println!("{}", o.area());

    println!("{}", o.wider());

    o.add();
    println!("{}", o);

}