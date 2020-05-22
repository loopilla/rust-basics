use std::ops::Mul;
use num_traits::ToPrimitive;

struct Vector2<T: Mul> {    // implements multiplication
    x: T,
    y: T
}

struct Vector21<U: Mul, V: Mul> {   // different types possible, both implements mul
    x: U,
    y: V
}

trait VOps<T> {
    fn magnitude(&self) -> f64;
    // fn x() -> T;
}

impl <T> VOps<T> for Vector2<T>
    // where T: Mul<T as Mul>::Output: ToPrimitive> {
    where T: Mul<Output = ToPrimitive> {
        fn magnitude(&self) -> f64 {
            let m = self.x * self.y;
            m.to_f64().unwrap()
            // (self.x * self.y) as f64
        }
}

fn main() {
    let s = Vector2 {x: 5, y: 6};

    println!("{}", s.magnitude());
}