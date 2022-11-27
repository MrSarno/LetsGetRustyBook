#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50
    };

    println!("The area of the rectangle is {} square pixels.", rect.area());

    println!("rect: {:#?}", rect); // use {:?} to opt-out of pretty-print, which uses multiple lines

    let rect1 = Rectangle {
        width: 20,
        height: 40
    };

    let rect2 = Rectangle {
        width: 40,
        height: 50
    };

    println!("\nrect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect can hold rect1: {}", rect.can_hold(&rect2));
}
