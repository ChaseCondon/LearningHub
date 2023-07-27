#[derive(Debug)] // Automatically creates debug information (used later to print)
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    // This is an associated function, not a method because it doesn't reference &self
    // This is related to the Rectangle, but doesn't require an already created Rectangle to be used
    // associated functions are often used for constructors as below, and often (but not always) are named "new"
    fn square(size: u32) -> Self { // Self here is an alias for Rectangle because we're in the impl block
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // Method vs field
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Associated functions are called with :: instead of . like methods
    // (Similar to static methods in Java)
    square = Rectangle::square(15);
}
