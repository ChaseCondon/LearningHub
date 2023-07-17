fn main() {
    {                             // s is not valid here, it’s not yet declared
        let s = "hello";    // s is valid from this point forward
        // do stuff with s
    }                             // this scope is now over, and s is no longer valid

    {
        let mut s = String::from("hello"); // Memory allocated on heap for variable s
        s.push_str(", world!");
        println!("{}", s);
    }
    // Variable s out of scope, memory owned by it is freed from heap
    // Rust auto calls String::drop() to do this at the point of the closing curly

    // Because primitive types have a known, fixed size and value, these are pushed onto the stack
    let x = 5;
    let y = x; // Makes a unique copy of the data at x - 5 in the stack

    // String will allocate memory in heap and store "hello" there
    // Will then put reference on stack to the ptr, len, and capacity for quick heap lookup
    let s1 = String::from("hello");
    // Copies stack reference to s2, but doesn't make a copy of the data in heap.
    // S2 now points to the same data that s1 does
    let s2 = s1; // s1 is invalidated as a variable to prevent double free (see below)
    // println!("{}", s1); // Compile error

    // Above causes a potential double free however, as when both variables go out of scope,
    // Rust will attempt to free the memory they both point to - that is, the same memory in heap
    // Given this, ln 24 is not a shallow copy of the reference, but rather a hard move of the reference
    // Deep copies must be done explicitly, if performance allows (i.e. using String::clone)

    let s1 = String::from("hello");
    let s2 = s1.clone(); // Makes a new alloc in heap and copies the data at &s1. Creates a new stack reference
    println!("s1 = {}, s2 = {}", s1, s2); // Now valid because s1 and s2 point to separate memory

    // Heap copies are not done by default because it is a more expensive task
    // given that their size and capacity are not known at compile time.
    // Primitives like i32 that are of known size and type can be quickly copied on the stack
    // thus why the below creates two valid variables
    let x = 5;
    let y = x; // Deep copy
    println!("x = {}, y = {}", x, y);

    let s = String::from("hello");   // s comes into scope

    takes_ownership(s);             // s's value moves into the function
                                                // ...and so is no longer valid here

    let x = 5;                             // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                                // but i32 is Copy, so it's ok to
                                                // still use x afterwards

    let s1 = gives_ownership();         // ownership of string created in function returned
                                                // and put in s1

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);  // Ownership of s2 is given to function, s2 becomes invalid
                                                        // Ownership of original data is then returned from function to s3
}

fn takes_ownership(some_string: String) { // some_string comes into scope, takes ownership from caller
    println!("{}", some_string);
} // some_string goes out of scope. `drop` is called

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // some_integer goes out of scope. Nothing special happens

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string                     // Ownership of some string is returned from the function
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // returns ownership of the received variable
}

