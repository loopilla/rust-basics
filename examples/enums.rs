#![allow(dead_code)]

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug)]
enum Direction {
    Up(Point),
    Down(Point),
    Left(Point),
    Right(Point)
}

#[derive(Debug)]
enum Keys {
    UpKey(String),
    DownKey(String),
    LeftKey(String),
    RightKey(String)
}

impl Direction {
    fn match_direction(&self) -> Keys {
        match *self {
            Direction::Up(_) => Keys::UpKey(String::from("Pressed w")),
            Direction::Down(_) => Keys::DownKey(String::from("Pressed s")),
            Direction::Left(_) => Keys::LeftKey(String::from("Pressed a")),
            Direction::Right(_) => Keys::RightKey(String::from("Pressed d")),
        }
    }
}

impl Keys {
    fn destruct(&self) -> &String {
        match *self {
            Keys::UpKey(ref s) => s,
            Keys::DownKey(ref s) => s,
            Keys::LeftKey(ref s) => s,
            Keys::RightKey(ref s) => s,
        }
    }
}

fn main() {
    let u = Direction::Up(Point {x: 0, y: -1});

    println!("{:?}", u.match_direction().destruct());
}