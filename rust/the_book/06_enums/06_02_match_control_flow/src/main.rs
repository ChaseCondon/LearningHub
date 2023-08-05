#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    // etc...
    Maryland,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


fn main() {
    let arizona_quarter = Coin::Quarter(UsState::Arizona);
    let maryland_quarter = Coin::Quarter(UsState::Maryland);

    value_in_cents(arizona_quarter);
    value_in_cents(maryland_quarter);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other),
        // _ => reroll() // default catch all. Can be given a variable name like above or ignored like here
        // Catch all MUST be last line since arms are evaluated in order
        _ => () // Tuple type catch all. Does nothing on mismatch
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// fn incomplete(x: Option<i32>) -> Option<i32> {
//     match x { // incomplete block. Match must cover all enum cases so needs a None arm
//         Some(i) => Some(i + 1),
//     }
// }

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll () {}