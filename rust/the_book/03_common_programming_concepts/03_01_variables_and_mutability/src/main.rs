fn main() {
    // let x = 5;
    // println!("The value of x is: {x}");
    // x = 6; // Doesn't work. x isn't mutable
    // println!("The value of x is: {x}");

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6; // Doesn't work. x isn't mutable
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let y = 5;
    let y = y + 1;
    {
        let y = y * 2; // Y shadows in the inner scope only
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is: {y}");
}
