use crate::IpAddrKind::V6;

enum IpAddrKind {
    V4,
    V6,
}

// enum can take values, making constructor functions
// values do not need to be of same type
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Can also use impl blocks with enums
impl Message {
    fn call(&self) {}
}

fn main() {
    // Both of these are of type IpAddrKind
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();
}

fn route(ip_kind: IpAddrKind) {}
