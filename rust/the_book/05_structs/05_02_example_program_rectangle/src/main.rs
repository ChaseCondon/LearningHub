#[derive(Debug)] // Automatically creates debug information (used later to print)
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!("rect1 is {:?}", rect1); // :? is a formatter to tell println to print the debug information provided by Debug
    println!("rect1 is {:#?}", rect1); // This is also a debug format, but this time multi-line (good for larger structs)

    dbg!(&rect1); // Can also use this to print debug info, but dbg takes ownership as opposed to println that takes reference
                  // dbg also prints to stderr instead of stdout

    let scale = 3;
    let rect2 = Rectangle {
        width: dbg!(30 * scale), // Gives us deeper debug info on how this field is derived
        // This is also fine because dbg returns ownership so value will still be valid
        height: 50,
    };

    dbg!(&rect2);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
