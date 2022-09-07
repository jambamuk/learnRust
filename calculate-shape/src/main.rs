#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self{
        Self { width: size , height: size }
    }
    fn area(&self) -> u32 {
        Self.width * self.height
    }
    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        if self.area() >= rectangle.area() {
            true
        } else {
            false
        }
    }
}
fn main() {
    let rectangle1 = Rectangle {
        width: 30,
        height: 60,
    };
    let rectangle2 = Rectangle {
        width: 40,
        ..rectangle1
    };
    let square = Rectangle::square(40);
    println!("square {:?}", square);
}
