mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Creates a shortcut to the hosting module
// Only works within the scope of the current module so doesn't make shortcut that customer can use
// Adding pub re-exports the alias so that external modules can use it as opposed to calling the entire path
pub use crate::front_of_house::hosting;

mod customer {
    use crate::front_of_house::hosting; // Need to create within the module to be usable
    use crate::front_of_house::hosting::add_to_waitlist; // This works too, but adds potential ambiguity

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist(); // were able to use just hosting instead of the whole path because of use
        add_to_waitlist(); // This also works, but adds ambiguity as to where the fn comes from. Above is more idiomatic
    }
}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}


// Exception for structs, enums is when the object we want to use shares a name in multiple packages
// use std::fmt;
// use std::io;

// fn function1() -> fmt::Result {
//     Result::Ok()
// }
//
// fn function2() -> io::Result<> {
//
// }

use std::fmt::Result;
use std::io::Result as IoResult; // We can also alias one of them if we want to not use parent