// use std::collections::HashMap; // With structs, enums its more idiomatic to use the full path
// use rand:Rng; // imports external trait from the rand crate

// Can also condense paths with shared roots
use std::{collections::HashMap, cmp::Ordering};
use std::io::{self, Write}; // self would give us std::io as an import

use std::collections::*; // This imports everything in collections

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    let mut list = LinkedList::new(); // brought in by the glob import off collections package

    // let secret_number = rand::thread_rng().gen_range(1..=100);
}