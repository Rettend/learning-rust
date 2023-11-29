fn main() {
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }

    let mut s = String::from("hello world");

    let word = first_word(&s);
    s.clear();

    // this works but the value of word is now not in sync with the value of s
    // this is a bug waiting to happen

    // this is where slices come in
    // a slice is a reference to a part of a string

    let s = String::from("hello world");
    // - s: R🟧, W🟦, O🟥

    let hello: &str = &s[0..5];
    // - s: R🟧
    let world: &str = &s[6..11];
    let s2: &String = &s;
    // - s: R🟧, W🟦, O🟥

    println!("{s2}");

    // because slices are references, the referenced variable loses W🟦 and O🟥 permissions

    // Range syntax

    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2]; // same as above

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..]; // same as above

    let slice = &s[0..len];
    let slice = &s[..]; // same as above

    // Rewriting first_word to use slices

    fn first_word2(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }

        &s[..]
    }

    let mut s = String::from("hello world");

    let word = first_word2(&s);
    // s.clear(); // error: cannot borrow `s` as mutable because it is also borrowed as immutable

    println!("the first word is: {}", word);

    // String literals are slices

    let s = "Hello, world!";

    // s is of type &str and is a slice pointing to that specific point of the binary
    // s is an immutable reference to a string literal

    // String Slices as Parameters

    // a better way to write the first_word function is to use &str instead of &String
    fn first_word3(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }

        &s[..]
    }

    // this way we can pass in both &String and &str values
    let my_string = String::from("hello world");
    let my_string_literal = "hello world";

    // we can pass string literals directly, whether partial or whole
    let word = first_word3(my_string_literal);
    let word = first_word3(&my_string[..6]);
    let word = first_word3(&my_string[..]);

    // we can pass String references
    let word = first_word3(&my_string);

    // Other Slices

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3]; // slice is of type &[i32]

    assert_eq!(slice, &[2, 3]);

    // Memory footprint of a slice

    // a slice is a "fat" pointer to the start of the slice and the length of the slice
    let s = String::from("hello");
    let s2: &String = &s; // 8 bytes
    let s3: &str = &s[..]; // 16 bytes (8 bytes for the pointer, 8 bytes for the length)

    println!(
        "&String={} &str={}",
        std::mem::size_of::<&String>(),
        std::mem::size_of::<&str>()
    )
}
