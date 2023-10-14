fn main() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    // Creating Instances of Structs

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.username = String::from("anotherusername567");

    // Field Init Shorthand

    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    // Creating Instances From Other Instances With Struct Update Syntax

    let user2 = User {
        email: String::from("another@example.com"),
        username: user1.username,
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    // <- user1 is dropped and user2 points to the same data

    let user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };

    // Tuple Structs Without Named Fields to Create Different Types

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Unit-Like Structs Without Any Fields

    struct AlwaysEqual;

    let subject = AlwaysEqual;

    // this is useful when implementing traits on some type but not storing any data in the type

    // Ownership of Struct Data

    // structs can contain references but we need to add lifetimes to the struct definition
    // struct User {
    //     username: &str,
    //     email: &str,
    //     active: bool,
    //     sign_in_count: u64,
    // }

    // let user1 = User {
    //     email: String::from("someone@example.com"),
    //     username: String::from("someusername123"),
    //     active: true,
    //     sign_in_count: 1,
    // };

    struct UserRef<'a> {
        username: &'a str,
        email: &'a str,
        active: bool,
        sign_in_count: u64,
    }

    let user1 = UserRef {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };

    // Borrowing Fields of a Struct

    // similar to tuples, the fields of a struct are tracked independently

    struct PointStruct {
        x: i32,
        y: i32,
    }

    let mut p = PointStruct { x: 0, y: 0 };
    // - p: R🟧, W🟦, O🟥
    // - p.x: R🟧, W🟦, O🟥
    // - p.y: R🟧, W🟦, O🟥

    // referencing p.x only removes permission from p and p.x but not p.y

    let x = &mut p.x;
    // - p: ❌
    // - p.x: ❌
    // - p.y: R🟧, W🟦, O🟥

    *x += 1; // - *x: R🟧, W🟦
             // - p: R🟧, W🟦, O🟥
             // - p.x: R🟧, W🟦, O🟥

    println!("{}, {}", p.x, p.y); // - p.x: R🟧, p.y: R🟧

    // but passing the whole struct to a function will remove all permissions from the struct and its fields

    fn print_point(p: &PointStruct) {
        println!("({}, {})", p.x, p.y);
    }

    let mut p = PointStruct { x: 0, y: 0 };
    // - p: R🟧, W🟦, O🟥
    // - p.x: R🟧, W🟦, O🟥
    // - p.y: R🟧, W🟦, O🟥

    let x = &mut p.x;
    // - p: ❌
    // - p.x: ❌
    // - p.y: R🟧, W🟦, O🟥

    // the reference to p.x removes the R🟧 that the function expects
    // print_point(&p); // - p: R🟧 error!

    *x += 1; // - *x: R🟧, W🟦
}
