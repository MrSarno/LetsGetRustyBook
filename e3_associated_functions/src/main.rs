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

impl Rectangle { // this could have instead gone into the previous impl block for Rectangle
    // this is just showing that there can be multiple
    fn square(size: u32) -> Rectangle {
        Rectangle{
            width: size,
            height: size
    }
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

    let _rect3 = Rectangle::square(25);

    println!("\nrect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect can hold rect1: {}", rect.can_hold(&rect2));
}
