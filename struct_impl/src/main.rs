#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, rect: Rectangle) -> bool {
        self.width >= rect.width && self.height >= rect.height
    }
}

fn main() {
    let rect0 = Rectangle {
        width: 20,
        height: 30,
    };
    println!("The area of the rectangle is {}", rect0.area());

    let rect1 = Rectangle {
        width: 40,
        height: 50,
    };
    println!("rect0 can hold the rect1: {}", rect0.can_hold(rect1));
}
