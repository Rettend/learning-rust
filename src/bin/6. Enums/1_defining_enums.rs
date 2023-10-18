// Defining an Enum

enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    // Enum Values

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    fn route(ip_kind: IpAddrKind) {};
    route(IpAddrKind::V4);
    route(four);

    // Enum with Data - Enum inside a Struct

    struct IpAddrStruct {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddrStruct {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddrStruct {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // Enum with Data

    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    enum IpAddr2 {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr2::V4(127, 0, 0, 1);

    enum Message {
        Quit,                       // No data associated
        Move { x: i32, y: i32 },    // Named fields
        Write(String),              // Enum with data
        ChangeColor(i32, i32, i32), // Enum with multiple parameters
    }

    impl Message {
        fn call(&self) {}
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    // Option Enum

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; error: mismatched types
}
