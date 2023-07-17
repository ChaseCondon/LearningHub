fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // Instead of passing ownership, we pass the reference to s1

    println!("The length of '{}' is {}.", s1, len); // Because we never passed ownership, s1 is still valid

    let mut s = String::from("hello");
    change(&mut s); // Because s is mutable, we can make a mutable reference
    println!("{s}");

    // let r1 = &mut s;
    // let r2 = &mut s; // Doesn't work, can only have one mut ref to a var at a time
    {
        let r1 = &mut s;
    }
    let r2 = &mut s; // This is ok because r2 went out of scope after the braces

    let mut s = String::from("hello");
    let s1 = &s;
    let s2 = &s; // This is okie because it doesn't matter how many people read only, no unintended behavior will occur
    let s3 = &mut s; // NOPE - can't have simultaneous mutable and immutable references

    let mut s = String::from("hello");
    let s1 = &s;
    let s2 = &s;
    println!("{s1} - {s2}"); // variables have been used. As this is last time scope ends and they are trahsed

    let s3 = &mut s; //this is okay because its now beyond the scope of s1/2
    println!("{}", s3);
}

fn calculate_length(s: &String) -> usize { // function receives reference to s and is thus able to access the data
    // Note that s doesn't point directly to the data in heap
    // but rather to the pointer in stack owned by s1
    // that in turn points to the data in heap
    s.len()
} // s is not dropped here because function doesn't own s, it's only borrowing the reference

/*
fn change(some_string: &String) {
   some_string.push_str(", world!"); // This doesn't work, you can't modify data you don't own
                                     // References are immutable by default
}
*/

fn change(some_string: &mut String) {        // This creates a mutable reference
    some_string.push_str(", world!"); // Because reference was mutable, we can edit the data
}