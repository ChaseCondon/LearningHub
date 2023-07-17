fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s[0..6]);
    let word = first_word(&s[..]);
    let word = first_word(&s); // Reference to a string is technically a string slice

    let string_literal = "Hello World";

    let lit_word = first_word(&string_literal[..]);
    let lit_word = first_word(string_literal); // string literals are also technically string slices

    // Not valid. Requires a mutable reference to s to clear it, but
    // there's already an immutable reference to the data in the slice returned from first_word
    // s.clear();

    println!("The first word of the phrase \"{s}\" is {word}");

    s.clear(); // This is now ok because the scope of word ended after being used in the println
}

// fn first_word(s: &String) -> &str {
fn first_word(s: &str) -> &str { // Now this works for both &String and &str
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

