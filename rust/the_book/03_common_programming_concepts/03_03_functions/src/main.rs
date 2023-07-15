fn main() {
    println!("Hello, world!");

    unit_labeled_measurement(5, 'h');

    let x = {
        let x = 3;
        x + 1;
    };

    let y = five();
    println!("The value returned from the five function is: {y}");

    let z = plus_one(y);
    println!("y + 1 = {z}")
}

fn unit_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    // you can use the 'return' keyword in rust, but the final line of a function is synonymous
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1 //note the lack of semicolon, which would make this a statement not an expression and thus fail
}