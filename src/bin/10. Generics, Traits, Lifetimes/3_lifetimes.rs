fn main() {
    // Preventing Dangling References with Lifetimes
    let r;

    {
        let x = 5;
        r = &x;
    }

    // println!("r: {r}"); // error! `x` does not live long enough

    // The Borrow Checker Ensures Data Outlives Its References

    // Generic Lifetimes in Functions

    // error! missing lifetime specifier: this function's return type contains a
    // borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
    // fn longest(x: &str, y: &str) -> &str {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }

    // Lifetime Annotation Syntax
    // &i32        // a reference
    // &'a i32     // a reference with an explicit lifetime
    // &'a mut i32 // a mutable reference with an explicit lifetime

    // Lifetime Annotations in Function Signatures
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    // println!("The longest string is {}", result); error! `string2` does not live long enough

    // Thinking in Terms of Lifetimes
    fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
        x
    }

    // Lifetime parameters don't change how long any of the references live
    // fn longest3<'a>(x: &str, y: &'a str) -> &'a str {
    //     let result = String::from("really long string");
    //     result.as_str()
    // }

    // Lifetime Annotations in Struct Definitions
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // Lifetime Elision
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    // Lifetime Annotations in Method Definitions
    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }

        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    // The Static Lifetime

    // string literals have the 'static lifetime by default
    let s: &'static str = "I have a static lifetime."; // this will live for the entire duration of the program

    // Generic Type Parameters, Trait Bounds, and Lifetimes Together
    use std::fmt::Display;

    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {ann}");
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
