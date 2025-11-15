fn main() {
    println!("Hello, world!");
    println!("");

    // variables and mutability
    let x = 5;
    let mut y = 10;
    println!("x = {}, y = {}", x, y);
    y += 2;
    println!("y after += 2: {}", y);

    // functions
    println!("add(2, 3) = {}", add(2, 3));

    // ownership: move
    let s = String::from("hello");
    takes_ownership(s); // s moved here
    // s is no longer usable after the move

    // borrowing
    let s2 = String::from("world");
    borrows(&s2); // borrow by reference
    println!("s2 still available: {}", s2);

    // struct and method
    let p = Point { x: 3, y: 4 };
    println!("Point: ({}, {})", p.x, p.y);
    println!("Distance from origin: {}", p.distance_from_origin());

    // enum + match
    let c = Color::Rgb(255, 0, 0);
    match c {
        Color::Rgb(r, g, b) => println!("RGB({}, {}, {})", r, g, b),
        Color::Named(name) => println!("Color name: {}", name),
    }

    // vectors and iterators
    let v = vec![1, 2, 3, 4];
    let doubled: Vec<_> = v.iter().map(|n| n * 2).collect();
    println!("doubled: {:?}", doubled);

    // closure
    let mul = |a: i32, b: i32| a * b;
    println!("3 * 4 = {}", mul(3, 4));

    // simple parse + error handling with Result
    match "42".parse::<i32>() {
        Ok(n) => println!("parsed: {}", n),
        Err(e) => println!("parse error: {}", e),
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn takes_ownership(s: String) {
    println!("took ownership: {}", s);
}

fn borrows(s: &String) {
    println!("borrowed: {}", s);
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance_from_origin(&self) -> f64 {
        let sum = self.x.pow(2) + self.y.pow(2);
        (sum as f64).sqrt()
    }
}

enum Color {
    Rgb(u8, u8, u8),
    Named(&'static str),
}

