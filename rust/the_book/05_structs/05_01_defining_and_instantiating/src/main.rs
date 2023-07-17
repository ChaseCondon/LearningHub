struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple structs - good for when your tuples need a named type
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual; // Unit-Like struct

fn main() {
    let mut user1 = build_user(
        String::from("someusername123"),
        String::from("someone@example.com"),
    );

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("yetanotheremail@example.com"),
        ..user1 // Copies fields from user1 that implement Copy, and moves the rest
                // Because String doesn't implement Copy, and thus username has been moved, user1 is no longer valid
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let x = origin.0; // Access field values with their index for tuple structs
    let y = origin.1;
    let z = origin.2;

    let subject = AlwaysEqual;
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username, // name is same, don't need to write full username: username
        email,
        sign_in_count: 1,
    }
}
