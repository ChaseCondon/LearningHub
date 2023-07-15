fn main() {
    number_compare_to_five(3);
    number_compare_to_five(7);
    number_compare_to_five(5);

    let condition = true;
    let number = if condition { 5 } else { 6 }; // Set to return value of expression

    let mut counter = 0;
    // infinite loop - can be returned from and assigned to a variable
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // breaks from infinite loop and returns counter * 2
        }
    };

    let mut count = 0;
    // loops can be given labels
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // and then labels explicitly broken - this will break from the outer loop
            }
            remaining -= 1;
        }

        count += 1;
    }

    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
}

fn number_compare_to_five(number: i32) {
    if number < 5 {
        println!("number {number} less than 5");
    } else if number > 5 {
        println!("number {number} greater than 5");
    } else {
        println!("number {number} equal to 5");
    }
}