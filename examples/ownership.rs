

fn main() {
    let s = String::from("String");

    // let _y = s;  // move ownership to y (s goes out of scope)

    let y = &s;    // borrowing s to y, when y goes out of scope, s gets back the ref

    println!("{}", s);

    println!("{}", y);
}