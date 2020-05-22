use num_traits::ToPrimitive;
use std::ops::Mul;
use rand::*;

trait Vector2Base<T> {
    fn len(&self) -> T;
}

struct Vector2<T> {
    x: T,
    y: T
}

impl <T> Vector2<T> {
    fn sqrt() -> T {
        f64::sqrt(to_i64(self.x) * to_i64(self.x) + (self.y * self.y))
    }
}

impl<T: Mul<Output=T>> Mul<T> for T {
    type Output = Self;
    fn mul (self, a: T) -> Self::Output {
        T(self * s)
    }
}

impl <T> Vector2Base<T> for Vector2<T> {
    fn len(&self) -> T {
        sqrt((self.x * self.x) + (self.y * self.y))
    }    
}

fn main() {
    let v = Vector2::<f32> {
        x: 2.0,
        y: 5.0
    };

}