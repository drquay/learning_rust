fn main() {
    let width: u32 = 30;
    let height: u32 = 50;
    println!("area: {}", area(width, height));
    println!("area_tuple: {}", area_tuple((width, height)));

    let scale: u32 = 3;
    let rec = Rectangle {width: dbg!(30*scale), height: 50};
    println!("area_structure: {}", area_structure(&rec));
    println!("Debug rec is {:?}", rec);
    println!("Format rec is {}", rec);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle::square(10);
    println!("rect2 is {}", rec);

    println!("rect1 contains rect2 : {}", rect1.can_hold(&rect2));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // associate func
    fn square(size: u32) -> Self {
        Rectangle {width: size, height: size}
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

use std::fmt;
impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "width: {}, height: {}", self.width, self.height)
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimentions: (u32, u32)) -> u32 {
    dimentions.0 * dimentions.1
}

fn area_structure(rec: &Rectangle) -> u32 {
    rec.width * rec.height
}

