// Vectors let you store multiple values next to each other in the heap

fn main() {
    let mut v: Vec<i32> = Vec::new(); // Declaring type because compiler can't determine on an empty Vec

    // let v = vec![1, 2, 3]; // Creation via macro, compiler is able to determine Vec<i32> type here

    // Updating
    for i in 1..9 {
        v.push(i);
    }
    println!("{:?}", v);

    let third: &i32 = &v[2]; // Direct reference (does not need to be a pointer). Panics when index doesn't exist
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2); // .get returns an optional of the type so can handle the index not existing
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let first = &v[0]; // creates immutable borrow of reference
    // v.push(9); // attempt to modify the original while immutable reference is in place
    println!("The first element is: {first}"); // panics
    // This happens because vectors must be in subsequent places in memory
    // If there isn't enough space at the current location at push, it will reallocate the whole Vec to new location
    // Then the immutable ref would point to the wrong place in memory

    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 50;
    }
}
