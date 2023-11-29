use std::fmt::format;

fn main() {
    // Creating a New String
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();

    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
    let hello = String::from("Csá tesó");

    // Updating a String
    // Appending to a String with push_str and push
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // does not take ownership of s2
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l');

    // Concatenation with the + Operator or the format! Macro
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used

    // The signature of the method that gets called when we use the + operator looks like this:
    // fn add(self, s: &str) -> String {}

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3; // s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}"); // does not move any of its parameters

    // Indexing into Strings
    let s1 = String::from("hello");
    // let h = s1[0]; // error: `std::string::String` cannot be indexed by `{integer}`

    let hello = String::from("hello");
    println!("{}", hello.len()); // 5

    let hello = String::from("안녕");
    println!("{}", hello.len()); // 6

    let hello = "안녕";
    // println!("{}", &hello[0]); // error

    println!("{}", hello.chars().nth(0).unwrap().escape_unicode()); // '\u{c548}'

    // Slicing Strings
    let hello = "안녕";

    println!("{}", &hello[0..3]); // 안
                                  // println!("{}", &hello[0..1]); // PANIC!

    // Methods for Iterating Over Strings
    for c in "안녕".chars() {
        print!("{c} ");
    }
    println!();

    for b in "안녕".bytes() {
        print!("{b} ");
    }
    println!();
}
