mod front_of_house; // declare module here so it pulls in from other file with same name
// only need to declare this file once in the crate, setting the modules parent
// other files would reference it from this one, i.e. crate::front_of_house

pub use crate::front_of_house::hosting; // uses module defined in front_of_house file

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}