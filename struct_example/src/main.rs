#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

fn main() {
    let height = 20;
    let width = 30;
    println!(
        "The area of the rectangle is {} square pixels",
        area(height, width)
    );

    let rect1 = (20, 30);
    println!(
        "The area of the rectangle is {} square pixels",
        area1(rect1)
    );

    let rect2 = Rectangle {
        height: 20,
        width: 30,
    };
    println!(
        "The area of the rectangle is {} square pixels",
        area2(&rect2)
    );
    println!("rectabgle is: {:?}", rect2);

    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: 20,
    };
    dbg!(&rect3);
}

fn area(height: u32, width: u32) -> u32 {
    height * width
}

fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area2(rectangle: &Rectangle) -> u32 {
    return rectangle.height * rectangle.width;
}
