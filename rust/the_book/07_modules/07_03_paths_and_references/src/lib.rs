// mod doesn't need to be pub because its in the same module as our pub fn
// so its within scope and available
mod front_of_house {
    // This however is now not in the same module as eat_as_restaurant
    // so it needs to be marked pub to be used
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    // making a struct pub does not make all its fields pub by default
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // but making the enum pub makes all variants pub as well
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // Super goes up one level to the parent
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    // Absolute path
    // crate specifically refers to the crate its being called from (kind of like self, but for the whole crate)
    // this would instead be the crate name in the case of external crates
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
    // Technically the above is the same as below wrt relative pathing
    // self::front_of_house::hosting::add_to_waitlist()
}

pub fn eat_again_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // toast is pub so we can access and edit it
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // This line won't compile because seasonal fruit isn't pub
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}